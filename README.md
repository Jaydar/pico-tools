# Port of elf2uf2 to rust

Implement log viewing and automatic deployment using a self-built CDC.

```Rust
async fn main(spawner: Spawner) {
    spawner.spawn(usb_cdc::init()).unwrap();
    //... 
}
```


```bash
cargo install pico-uf2
pico-uf2 --before reboot --after test  -input ./test.elf -output ./g/test.uf2
```



Original at https://github.com/JoNil/elf2uf2-rs
Original at https://github.com/raspberrypi/pico-sdk/tree/master/tools/elf2uf2
