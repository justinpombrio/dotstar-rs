use crate::color::*;
use crate::int_math::{cos, inc, inc_i32, sin};
use crate::lights::*;

/// A strobing light show.
pub struct StrobeShow {
    brightness: i8, // brightness of one of the two colors (the other is max)
    hue: i32,       // the hue of the same color (the other has opposite hue)
    delay: i32,     // the duration of each stage, in ms
    state: bool,    // color 1 or color 2?
}

impl StrobeShow {
    pub fn preset1(&mut self) {
        self.brightness = 70;
        self.hue = 0;
        self.delay = 40;
    }

    pub fn preset2(&mut self) {
        self.brightness = 70;
        self.hue = 90;
        self.delay = 40;
    }

    pub fn preset3(&mut self) {
        self.brightness = 0;
        self.hue = 0;
        self.delay = 40;
    }

    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.brightness, delta, 0, 100);
    }

    pub fn change_hue(&mut self, delta: i32) {
        inc_i32(&mut self.hue, delta, -180, 180);
    }

    pub fn change_delay(&mut self, delta: i32) {
        inc_i32(&mut self.delay, delta, 5, 1000);
    }
}

impl LightShow for StrobeShow {
    fn new() -> StrobeShow {
        StrobeShow {
            brightness: 70,
            hue: 0,
            delay: 40,
            state: false,
        }
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        self.state = !self.state;
        self.update(lights);
        Duration::Millis(self.delay as u32)
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        let color = if self.state {
            let a = sin(self.hue, 40) as i8;
            let b = cos(self.hue, 40) as i8;
            ColorLab {
                l: self.brightness,
                a,
                b,
            }
            .to_srgb_clamped()
        } else {
            let a = sin(180 + self.hue, 40) as i8;
            let b = cos(180 + self.hue, 40) as i8;
            ColorLab { l: 70, a, b }.to_srgb_clamped()
        };
        for light in lights {
            *light = color;
        }
    }
}
