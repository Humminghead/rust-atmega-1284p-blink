# Atmega-1284p blink led example on RUST
This is simple project on RUST for atmega-1284p atmel MCU. The main goal of this project is try to use RUST language in debug mode with old Atmel MCU's. I did it successfully.

In `.vscode` folder you may find my steps to run project in debug mode (see `tasks.json` and `launch.json`).

## Toolchain:

- [ATMEGA1284P-XPLANED](https://www.microchip.com/en-us/development-tool/ATMEGA1284P-XPLD) board
- [VsCode](https://code.visualstudio.com/download) with `Rust Syntax`, `rust-analyser` and `Even Better TOML` extensions
- [avr-gcc](https://gcc.gnu.org/wiki/avr-gcc)
- [avrdude](https://github.com/avrdudes/avrdude)
- [avarice](https://linux.die.net/man/1/avarice)
- [RUST](https://www.rust-lang.org) nightly toolchain 2021-01-07

## Troubles:
I ran into some issues with the toolchain version. I hope they get resolved soon by RUST developers (see links in my code). But now everything works in this project.
