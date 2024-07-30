# Automatic deployment

Implement log viewing and automatic deployment using a self-built CDC.

```Rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(usb_cdc::init()).unwrap();
    // ...... 
}
```


```bash
cargo install pico-uf2
pico-uf2 --before reboot --after clear  -input ./test.elf -output ./g/test.uf2
```
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

