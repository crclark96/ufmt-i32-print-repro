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

    for x in (5..100).step_by(5) {
        for y in 1..5 {
            let z = x / y;
            ufmt::uwriteln!(&mut serial, "{} / {} = {}\r", x, y, z).void_unwrap();
        }
    }
    loop {}
}
