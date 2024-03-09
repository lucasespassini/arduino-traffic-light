#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut red_led = pins.d11.into_output();
    let mut yellow_led = pins.d8.into_output();
    let mut green_led = pins.d6.into_output();

    loop {
        green_led.toggle();
        arduino_hal::delay_ms(5000);
        green_led.toggle();

        yellow_led.toggle();
        arduino_hal::delay_ms(2000);
        yellow_led.toggle();

        red_led.toggle();
        arduino_hal::delay_ms(5000);
        red_led.toggle();
    }
}
