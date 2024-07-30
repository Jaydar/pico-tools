#![no_std]
#![no_main]

use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};
use rp2040_hal::rom_data::reset_to_usb_boot;
use {defmt_rtt as _, panic_probe as _};

// 绑定中断处理程序
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
pub async fn init() {
    // 初始化RP2040外围设备
    let p = embassy_rp::init(Default::default());

    // 从HAL创建USB驱动程序
    let driver = Driver::new(p.USB, Irqs);

    // 创建embassy-usb配置
    let mut config = Config::new(65535u16, 65535u16);
    config.manufacturer = Some("Jaydar");
    config.product = Some("RP2040 Debug");
    config.serial_number = Some("RP2040");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // 为了兼容Windows，需要设置如下配置
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // 使用驱动程序和配置创建embassy-usb DeviceBuilder
    // 需要一些缓冲区来构建描述符
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // 没有msos描述符
        &mut control_buf,
    );

    // 在构建器上创建类
    let class = CdcAcmClass::new(&mut builder, &mut state, 64);

    // 构建USB设备
    let mut usb = builder.build();

    // 将CDC-ACM类分成独立的TX和RX端点
    let (mut usb_tx, mut usb_rx) = class.split();

    // 运行USB设备
    let usb_fut = usb.run();

    // 处理USB接收并回显数据的Future
    let usb_echo_fut = async {
        let mut buf = [0; 64];
        let mut message = [0; 128];
        let mut idx = 0;

        loop {
            // 读取接收到的数据
            let n = match usb_rx.read_packet(&mut buf).await {
                Ok(n) => n,
                Err(EndpointError::Disabled) => break,
                Err(_) => continue,
            };

            // 回显接收到的每个字节
            for &byte in &buf[..n] {
                usb_tx.write_packet(&[byte]).await.ok();

                // 如果接收到换行符，则附加 "Hi"
                if byte == b'\r' || byte == b'\n' {
                    if idx > 0 {
                        usb_tx.write_packet(b"\r\n").await.ok();

                        let is_boot = idx == 6
                            && message[0] == b'r'
                            && message[1] == b'e'
                            && message[2] == b'b'
                            && message[3] == b'o'
                            && message[4] == b'o'
                            && message[5] == b't';

                        let is_clear = idx == 5
                            && message[0] == b'c'
                            && message[1] == b'l'
                            && message[2] == b'e'
                            && message[3] == b'a'
                            && message[4] == b'r';

                        if is_boot {
                            usb_tx.write_packet(b"Booting\r\n").await.ok();
                            Timer::after_secs(1).await;
                            reset_to_usb_boot(0, 0)
                        } else if is_clear {
                            usb_tx.write_packet(b"\x1b[2J\x1b[H").await.ok();
                        } else {
                            usb_tx.write_packet(&message[..idx]).await.ok();
                            usb_tx.write_packet(b" Hi\r\n").await.ok();
                        }

                        idx = 0;
                    }
                } else {
                    // 存储接收到的字符
                    if idx < message.len() {
                        message[idx] = byte;
                        idx += 1;
                    }
                }
            }
        }
    };

    // 并发运行所有任务
    embassy_futures::join::join(usb_fut, usb_echo_fut).await;
}
