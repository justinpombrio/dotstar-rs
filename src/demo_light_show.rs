use crate::color::*;
use crate::lights::*;

const WAVELEN: isize = 60;
const AMPLITUDE: isize = 80;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoSettings {
    brightness: i8,
}

impl DemoSettings {
    pub fn new() -> DemoSettings {
        DemoSettings { brightness: 50 }
    }

    pub fn inc(&mut self) {
        if self.brightness < 100 {
            self.brightness += 10;
        }
    }

    pub fn dec(&mut self) {
        if self.brightness >= 10 {
            self.brightness -= 10;
        }
    }
}

#[derive(Debug)]
pub struct Demo {
    settings: DemoSettings,
    state: isize,
}

impl LightShow for Demo {
    type Settings = DemoSettings;

    fn new(settings: &DemoSettings) -> Demo {
        Demo {
            settings: *settings,
            state: 0,
        }
    }

    fn update_settings(&mut self, settings: &DemoSettings) {
        self.settings = *settings;
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        self.state += 1;
        let deg = 360 * self.state / WAVELEN;
        let l = self.settings.brightness;
        let a = sin(deg, AMPLITUDE / 2);
        let b = cos(deg, AMPLITUDE / 2);
        lights[0] = lab(99, 0, 0);
        lights[1] = lab(l, -b as i8, a as i8);
        lights[2] = lab(l, a as i8, b as i8);
        lights[3] = lab(l, b as i8, -a as i8);
        lights[4] = lab(l, -a as i8, -b as i8);
        Duration::Millis(100)
    }
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
    ColorLab { l, a, b }.to_srgb_clamped()
}
