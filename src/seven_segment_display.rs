use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

pub struct SevenSegmentDisplay {
    pub a: Pin<Output>,
    pub b: Pin<Output>,
    pub c: Pin<Output>,
    pub d: Pin<Output>,
    pub e: Pin<Output>,
    pub f: Pin<Output>,
    pub g: Pin<Output>,
    pub dp: Pin<Output>,
}

impl SevenSegmentDisplay {
    pub fn light_segment(&mut self, segment: char) {
        match segment {
            'a' => self.a.set_high(),
            'b' => self.b.set_high(),
            'c' => self.c.set_high(),
            'd' => self.d.set_high(),
            'e' => self.e.set_high(),
            'f' => self.f.set_high(),
            'g' => self.g.set_high(),
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.a.set_low();
        self.b.set_low();
        self.c.set_low();
        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
    }

    //   ___
    //    A
    // |F   B|
    //    G
    // |E   C|
    //    D

    pub fn digit_0(&mut self) {
        self.clear();
        self.light_segment('a');
        self.light_segment('b');
        self.light_segment('c');
        self.light_segment('d');
        self.light_segment('e');
        self.light_segment('f');
    }

    pub fn digit_1(&mut self) {
        self.clear();
        self.light_segment('b');
        self.light_segment('c');
    }

    pub fn digit_2(&mut self) {
        self.clear();
        self.light_segment('a');
        self.light_segment('b');
        self.light_segment('g');
        self.light_segment('e');
        self.light_segment('d');
    }

    pub fn digit_3(&mut self) {
        self.digit_1();
        self.light_segment('a');
        self.light_segment('g');
        self.light_segment('d');
    }

    pub fn digit_4(&mut self) {
        self.digit_1();
        self.light_segment('f');
        self.light_segment('g');
    }

    pub fn digit_5(&mut self) {
        self.clear();
        self.light_segment('a');
        self.light_segment('f');
        self.light_segment('g');
        self.light_segment('c');
        self.light_segment('d');
    }

    pub fn digit_6(&mut self) {
        self.digit_8();
        self.b.set_low();
    }

    pub fn digit_7(&mut self) {
        self.digit_1();
        self.light_segment('a');
    }

    pub fn digit_8(&mut self) {
        self.digit_0();
        self.light_segment('g');
    }
    //   ___
    //    A
    // |F   B|
    //    G
    // |E   C|
    //    D
    pub fn digit_9(&mut self) {
        self.clear();
        self.light_segment('a');
        self.light_segment('b');
        self.light_segment('c');
        self.light_segment('d');
        self.light_segment('g');
        self.light_segment('f');
    }
}
