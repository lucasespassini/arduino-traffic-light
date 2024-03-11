//   ___
//    A
// |F   B|
//    G
// |E   C|
//    D

pub struct SevenSegmentDisplay {
    a: Pin<Output>,
    b: Pin<Output>,
    c: Pin<Output>,
    d: Pin<Output>,
    e: Pin<Output>,
    f: Pin<Output>,
    g: Pin<Output>,
    dp: Pin<Output>,
}

impl SevenSegmentDisplay {
    pub fn new(pins: arduino_hal::Pins) -> Self {
        SevenSegmentDisplay {
            a: pins.d10.into_output(),
            b: pins.d11.into_output(),
            c: pins.d6.into_output(),
            d: pins.d5.into_output(),
            e: pins.d4.into_output(),
            f: pins.d9.into_output(),
            g: pins.d8.into_output(),
            dp: pins.d7.into_output(),
        }
    }

    // Função para acender um segmento específico
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

    // Função para apagar todos os segmentos
    pub fn clear(&mut self) {
        self.a.set_low();
        self.b.set_low();
        self.c.set_low();
        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
    }
}
