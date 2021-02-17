#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "unsigned 0..80_000\r").void_unwrap();

    for x in (0u32..80_000u32).step_by(10000) {
        ufmt::uwriteln!(&mut serial, "{}\r", x).void_unwrap();
    }

    ufmt::uwriteln!(&mut serial, "signed 0..80_000\r").void_unwrap();

    for x in (0i32..80_000i32).step_by(10000) {
        ufmt::uwriteln!(&mut serial, "{}\r", x).void_unwrap();
    }

    loop {}
}
