# stm32f303-rtfm-usb-example
A simple exampel of a USB device (acm_cdc) on a stm32f303vc-discovery utelice rtfm (real time for the masses).

## run
To run the example, complie the prodjekt `cargo build --release`.
"--release" is nesesary dud to usbs strict timing requierment.
Set upp a openocd conection `openocd -f openocd.cfg`.

In a new window run `cargo run --release` to lanch the example on the Discovery board.
Finaly launch `sudo screen /dev/ttyACM1 115200`.
Try typing in the window.
You should resev the caracter in its capital form as return.
You should allso se the south LED blick red as you type.
And in the openocd window should you se a "ok" print for every sucessfull transmition.

## Common errors
If it don't buid: check if you hav the target "thumbv7em-none-eabihf" with `rustup target --list`.
If not install using `rustup tartget --add thumbv7em-none-eabihf`.

If `cargo run` fails: check `config` and `.cargo/config` sow it uses the some version of gdb as you have installd.

If "screen" don't work: check 'dmesg' for dbug msgs.
Hass the device concted corectly?
Is the device conected to "ttyACM1"?
Is it a ACM device?

