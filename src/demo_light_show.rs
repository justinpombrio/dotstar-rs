use crate::lights::*;


pub struct Demo {
    state: usize
}

impl LightShow for Demo {
    fn new() -> Demo {
        Demo {
            state: 0
        }
    }

    fn next(&mut self) -> Lights {
        self.state += 1;
        let a = 50 * (self.state % 6) as u8;
        let b = 50 * ((self.state + 1) % 6) as u8;
        let c = 50 * ((self.state + 2) % 6) as u8;
        Lights {
            lights: [
                Light::new(a, b, a),
                Light::new(b, b, c),
                Light::new(a, c, c)
            ]
        }
    }
}
