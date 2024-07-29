# Port of elf2uf2 to rust

```bash
cargo install pico-uf2


pico-uf2 --before reboot --after test  -input ./test.elf -output ./g/test.uf2
pico-uf2 -b reboot  -a test -i ./test.elf -o ./g/test.uf2
pico-uf2 -b -a 


```

## Options
-d automatic deployment to a mounted pico.
-s open the pico as a serial device after deploy and print serial output.


Original at https://github.com/JoNil/elf2uf2-rs
Original at https://github.com/raspberrypi/pico-sdk/tree/master/tools/elf2uf2