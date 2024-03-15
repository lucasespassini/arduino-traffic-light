#![no_std]
#![no_main]

mod seven_segment_display;
use seven_segment_display::SevenSegmentDisplay;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut red_led = pins.d0.into_output();
    let mut yellow_led = pins.d1.into_output();
    let mut green_led = pins.d2.into_output();

    let mut display = SevenSegmentDisplay {
        a: pins.d10.downgrade().into_output(),
        b: pins.d11.downgrade().into_output(),
        c: pins.d6.downgrade().into_output(),
        d: pins.d5.downgrade().into_output(),
        e: pins.d4.downgrade().into_output(),
        f: pins.d9.downgrade().into_output(),
        g: pins.d8.downgrade().into_output(),
        dp: pins.d7.downgrade().into_output(),
    };

    let mut counter: u8 = 9;
    let mut is_open_traffic_light = true;

    loop {
        if is_open_traffic_light {
            green_led.set_high();
            red_led.set_low();
        } else {
            green_led.set_low();
            red_led.set_high();
        }

        match counter % 10 {
            0 => display.digit_0(),
            1 => display.digit_1(),
            2 => display.digit_2(),
            3 => display.digit_3(),
            4 => display.digit_4(),
            5 => display.digit_5(),
            6 => display.digit_6(),
            7 => display.digit_7(),
            8 => display.digit_8(),
            9 => display.digit_9(),
            _ => panic!("impossible case"),
        }

        // counter = counter.overflowing_add(1).0;
        counter -= 1;

        if counter == 0 && yellow_led.is_set_low() {
            counter = 9;

            if is_open_traffic_light {
                display.clear();
                green_led.set_low();
                yellow_led.set_high();
                arduino_hal::delay_ms(2000);
                yellow_led.set_low();
            }

            is_open_traffic_light = !is_open_traffic_light;
        } else {
            arduino_hal::delay_ms(1000);
        }
    }
}
