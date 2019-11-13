# stm32f303-rtfm-usb-example
A simple example of a USB device (acm_cdc) on a stm32f303vc-discovery utilize rtfm (real time for the masses).

## run
To run the example, compile the project `cargo build --release`.
"--release" is necessary dud to usbs strict timing requirements.
Set up a openocd connection `openocd -f openocd.cfg`.

In a new window run `cargo run --release` to launch the example on the Discovery board.
Finally launch `sudo screen /dev/ttyACM1 115200`.
Try typing in the window.
You should reserve the character in its capital form as return.
You should all sow see the south LED blink red as you type.
And in the openocd window should you see a "ok" print for every successful transition.

## Common errors
If it don't build: check if you have the target "thumbv7em-none-eabihf" with `rustup target --list`.
If not install using `rustup tartget --add thumbv7em-none-eabihf`.

If `cargo run` fails: check `config` and `.cargo/config` sow it uses the some version of gdb as you have installed.

If "screen" don't work: check 'dmesg' for debug msgs.
Has the device connected correctly?
Is the device connected to "ttyACM1"?
Is it a ACM device?

