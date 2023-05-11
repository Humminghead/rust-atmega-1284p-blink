#![no_std]
#![no_main]
#![feature(lang_items)]

// https://doc.rust-lang.org/error_codes/E0432.html
extern crate panic_halt;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

fn simple_sleep(count: u32) {
    let mut counter: u32 = count;
    while counter > 0 {
        counter = counter - 1;
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let wait_count = 100000;

    let dp = avr_device::atmega1284p::Peripherals::take().unwrap();
    dp.PORTB.ddrb.write(|w| w.pb3().set_bit());

    loop {
        // Clear bit 5 of port B:
        dp.PORTB.portb.write(|w| w.pb3().clear_bit());

        //Sleep
        simple_sleep(wait_count);

        // Set bit 5 of port B:
        dp.PORTB.portb.write(|w| w.pb3().set_bit());

        //Sleep
        simple_sleep(wait_count)
    }
}
