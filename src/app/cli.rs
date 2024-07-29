
use clap::Parser;

/// Command line arguments for the pico-uf2 tool
#[derive(Parser, Debug)]
#[command(name = "pico-uf2", version = "1.0", author = "Jaydar", about = "elf to uf2, Automatically restart to enter BootSLE")]
pub struct Args {

        /// Input file
    #[arg(value_name = "INPUT", help = "Input file")]
    pub input: String,

    /// Start command sent to COM port, default is "Reboot"
    #[arg(short, long, value_name = "BEFORE", help = "Start command sent to COM port, default is \"Reboot\"")]
    pub before: Option<String>,

    /// End command sent to COM port, default is not to send a command
    #[arg(short, long, value_name = "AFTER", help = "Connect to COM after operation, and send command to COM")]
    pub after: Option<String>,

    /// UF2 file output location
    #[arg(short, long, value_name = "OUTPUT", help = "UF2 file output location")]
    pub output: Option<String>,
}