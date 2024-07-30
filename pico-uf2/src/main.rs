mod app;

use std::{error::Error, fs::File, io::{BufReader, BufWriter}, thread::sleep, time::Duration};

use app::{cdc, elf_to_uf2};
use clap::Parser;
use sysinfo::{DiskExt, SystemExt};



fn main() -> Result<(), Box<dyn Error>> {
    
    let args = app::cli::Args::parse();
    println!("{:?}", args);
    if let Some(before) = args.before {
        app::cdc::send(before)?;
    }
    
    sleep(Duration::from_secs(1));

    let input = BufReader::new(File::open(args.input)?);
    let output = {
        let sys = sysinfo::System::new_all();

        let mut pico_drive = None;
        for disk in sys.disks() {
            let mount = disk.mount_point();

            if mount.join("INFO_UF2.TXT").is_file() {
                println!("Found pico uf2 disk {}", &mount.to_string_lossy());
                pico_drive = Some(mount.to_owned());
                break;
            }
        }

        if let Some(pico_drive) = pico_drive {
            File::create(pico_drive.join("out.uf2"))?
        } else {
            return Err("Unable to find mounted pico".into());
        }
    };

    if let Err(err) = elf_to_uf2::elf_to_uf2(input, BufWriter::new(output)) {
        return Err(err);
    }


    if let Some(after) = args.after {
        sleep(Duration::from_secs(1));
        cdc::connect(after)?;
    }
    

    Ok(())
}