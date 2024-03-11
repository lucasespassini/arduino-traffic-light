#![no_std]
#![no_main]

// mod seven_segment_display;
// use seven_segment_display::SevenSegmentDisplay;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut red_led = pins.d1.into_output();
    let mut yellow_led = pins.d2.into_output();
    let mut green_led = pins.d3.into_output();

    // let mut display = SevenSegmentDisplay::new(pins);

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
