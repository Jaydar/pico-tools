# Automatic deployment

PR2040 Rust automatic deployment tool. Using **[pico-uf2](https://crates.io/crates/pico-uf2)** and **[pico-cdc](https://crates.io/crates/pico-cdc)** together enables automatic deployment.

1.install

```bash
cargo install pico-uf2
cargo add pico-cdc
```
2.modify code
```Rust

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(usb_cdc::init()).unwrap();
    // ...... 
}
```

**[Complete the example](https://github.com/Jaydar/pico-tools/tree/master/example)** 

## Option
```
Arguments:
  <INPUT>  Input file

Options:
  -b, --before <BEFORE>  Start command sent to COM port, default is "Reboot"
  -a, --after <AFTER>    Connect to COM after operation, and send command to COM
  -o, --output <OUTPUT>  UF2 file output location
  -h, --help             Print help
  -V, --version          Print version
```


## Acknowledgements and Copyright

This project includes code borrowed or directly used from the following open-source projects:

*  **[embassy](https://github.com/embassy-rs/embassy)** 

*  **[rp2040-hal](https://github.com/rp-rs/rp-hal)** 

*  **[elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs)** 

*  **[pico-sdk](https://github.com/raspberrypi/pico-sdk/tree/master/tools/elf2uf2)**

