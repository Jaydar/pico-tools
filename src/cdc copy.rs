static OPTS: OnceCell<Opts> = OnceCell::new();

fn send_cdc_command(port_name: &str, command: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_secs(1))
        .open()?;

    // 在命令后附加回车和换行符
    let mut command_with_newline = command.to_vec();
    command_with_newline.push(b'\r');
    command_with_newline.push(b'\n');

    port.write_all(&command_with_newline)?;

    // 等待设备的反馈
    // thread::sleep(Duration::from_millis(100));

    let mut serial_buf = [0; 1024];
    match port.read(&mut serial_buf) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                println!(
                    "Received feedback from {}: {}",
                    port_name,
                    String::from_utf8_lossy(&serial_buf[..bytes_read])
                );
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
            println!("No feedback received from {}", port_name);
        }
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

fn reboot() -> Result<(), Box<dyn Error>> {
    let ports = serialport::available_ports()?;
    let usb_ports: Vec<_> = ports.iter()
        .filter(|p| matches!(p.port_type, serialport::SerialPortType::UsbPort(_)))
        .collect();

    if usb_ports.is_empty() {
        println!("No connected Pico found");
        return Ok(());
    }

    for pico_port in usb_ports {
        match send_cdc_command(&pico_port.port_name, b"reboot") {
            Ok(_) => println!("Reboot command sent to {}", pico_port.port_name),
            Err(e) => println!("Failed to send reboot command to {}: {}", pico_port.port_name, e),
        }
    }
    
    Ok(())
}


