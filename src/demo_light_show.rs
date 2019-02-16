use crate::color::*;
use crate::lights::*;

const WAVELEN: isize = 60;
const AMPLITUDE: isize = 40;

pub struct Demo {
    state: isize,
}

fn sin(deg: isize, multiplier: isize) -> isize {
    if deg < 0 {
        return -sin(-deg, multiplier);
    }
    let deg = deg % 360;
    if deg > 180 {
        return -sin(360 - deg, multiplier);
    }
    // Thanks, Bhaskara I
    multiplier * 4 * deg * (180 - deg) / (40500 - deg * (180 - deg))
}

fn cos(deg: isize, multiplier: isize) -> isize {
    sin(90 - deg, multiplier)
}

fn lab(l: i8, a: i8, b: i8) -> ColorRgb {
    let ColorRgb { r, g, b } = ColorLab { l, a, b }.to_srgb_clamped();
    ColorRgb { r, g, b }
}

impl Demo {
    pub fn new() -> Demo {
        Demo { state: 0 }
    }
}

impl LightShow<()> for Demo {
    fn update_settings(&mut self, _: &()) {}

    fn next(&mut self, lights: &mut [ColorRgb]) -> Timeout {
        self.state += 1;
        let deg = 360 * self.state / WAVELEN;
        let a = sin(deg, AMPLITUDE / 2);
        let b = cos(deg, AMPLITUDE / 2);
        lights[0] = lab(100, 0, 0);
        lights[1] = lab(80, 0, 20);
        lights[2] = lab(80, a as i8, b as i8);
        lights[3] = lab(80, -20, 0);
        lights[4] = lab(80, 0, -20);
        Timeout::Millis(100)
    }
}
