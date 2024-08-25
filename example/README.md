# Example

## Run

Manual entry into BootSEL mode is required for the first use.

```bash

cargo install pico-uf2
cargo run

```
Upon restarting:

1. Notify CDC to send the reboot command to enter BootSEL mode,
2. Flash the firmware,
3. Connect to CDC and send the clear command.




