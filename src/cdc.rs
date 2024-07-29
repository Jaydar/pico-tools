use std::{error::Error, io::{self, Write}, thread::sleep, time::Duration};

use serialport::SerialPortInfo;

pub fn get_usb_ports()->Result<Vec<SerialPortInfo>, Box<dyn Error>>{
    let ports = serialport::available_ports()?;
    let usb_ports: Vec<SerialPortInfo> = ports.into_iter()
        .filter(|p| matches!(p.port_type, serialport::SerialPortType::UsbPort(_)))
        .collect();

    if usb_ports.is_empty() {
        println!("No connected Pico found");
        // return Err("")
    }

    return Ok(usb_ports);
    
}

pub fn send(command:String) -> Result<(), Box<dyn Error>>{
    let usb_ports = get_usb_ports()?;
    for item in usb_ports {
        let mut port = serialport::new(&item.port_name, 115200)
        .timeout(Duration::from_secs(1))
        .open()?;
         // 在命令后附加回车和换行符
        let mut command_with_newline = command.as_bytes().to_vec();
        command_with_newline.push(b'\r');
        command_with_newline.push(b'\n');

        
        port.write_all(&command_with_newline)?;

        
        sleep(Duration::from_millis(100));
        
        
        let mut serial_buf = [0; 1024];
        match port.read(&mut serial_buf) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    println!(
                        "Received feedback from {}: {}",
                        &item.port_name,
                        String::from_utf8_lossy(&serial_buf[..bytes_read])
                    );
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("error {:?}",e);
                println!("No feedback received from {}", &item.port_name);
            }
            Err(e) => {
                println!("error {:?}",e);
                return Err(Box::new(e))
            },
        }
        

    }
    Ok(())

}

pub fn connect(command:String) -> Result<(),Box<dyn Error>>{
    let usb_ports = get_usb_ports()?;

    for port in usb_ports {
        let mut port = serialport::new(&port.port_name, 115200)
                    .timeout(Duration::from_millis(100))
                    .flow_control(serialport::FlowControl::None)
                    .open()?;
        

        if port.write_data_terminal_ready(true).is_ok() {
            
            let mut command_with_newline = command.as_bytes().to_vec();
            command_with_newline.push(b'\r');
            command_with_newline.push(b'\n');
            port.write_all(&command_with_newline)?;
            
            let mut serial_buf = [0; 1024];
            loop {
                match port.read(&mut serial_buf) {
                    Ok(t) => {
                        io::stdout().write_all(&serial_buf[..t])?;
                        io::stdout().flush()?;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }

    Ok(())   
}