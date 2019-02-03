use crate::lights::*;

const WAVELEN: isize = 20;

pub struct Demo {
    state: isize,
}

impl Demo {
    fn wave(d: isize) -> u8 {
        (256 / WAVELEN) as u8 * isize::abs(d % WAVELEN - WAVELEN / 2) as u8
    }
}

impl LightShow for Demo {
    fn new() -> Demo {
        Demo { state: 0 }
    }

    fn next(&mut self) -> Lights {
        self.state += 1;
        let a = Demo::wave(self.state);
        let b = Demo::wave(self.state + WAVELEN / 4);
        let c = Demo::wave(self.state + WAVELEN / 2);
        Lights {
            lights: [
                Light::new(a, b, c),
                Light::new(b, c, a),
                Light::new(c, a, b),
            ],
        }
    }
}
