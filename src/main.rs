#![no_std]
#![no_main]
#![feature(lang_items)]

// https://doc.rust-lang.org/error_codes/E0432.html
extern crate panic_halt;

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
// #[cfg(debug_assertions)]
// use panic_halt as _;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
use panic_abort as _;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn main() {
    loop {
        // avr_device::generic
        // avr_device::atmega1284p
    }
}
