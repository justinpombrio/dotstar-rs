use crate::color::*;
use crate::lights::*;

//const WAVELEN: isize = 20;

pub struct Demo {
    //    state: isize,
}

impl Demo {
    /*    fn wave(d: isize) -> u8 {
        (256 / WAVELEN) as u8 * isize::abs(d % WAVELEN - WAVELEN / 2) as u8
    }*/
}

fn lab(l: i8, a: i8, b: i8) -> Rgb {
    let ColorRgb { r, g, b } = ColorLab { l, a, b }.into();
    Rgb { r, g, b }
}

impl Demo {
    pub fn new() -> Demo {
        Demo {}
        //        Demo { state: 0 }
    }
}

impl LightShow<()> for Demo {
    fn update_settings(&mut self, _: &()) {}

    fn next(&mut self, lights: &mut [Rgb]) -> Timeout {
        lights[0] = lab(80, 20, 0);
        lights[1] = lab(80, -20, 0);
        lights[2] = lab(80, 0, 20);
        lights[3] = lab(80, 0, -20);
        lights[4] = lab(80, 20, 20);
        Timeout::Never
        /*
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
        */
    }
}
