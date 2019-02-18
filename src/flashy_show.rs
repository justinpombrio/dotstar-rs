use crate::color::*;
use crate::int_math::{cos, inc, sin};
use crate::lights::*;

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct FlashyShow {
    radius: i8,
    brightness: i8,
    state: isize,
}

impl FlashyShow {
    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.brightness, delta, 0, 99);
        self.radius = ColorLab {
            l: self.brightness,
            a: 0,
            b: 0,
        }
        .max_radius();
    }
}

impl LightShow for FlashyShow {
    fn new() -> FlashyShow {
        FlashyShow {
            state: 0,
            brightness: 70,
            radius: 40,
        }
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state
        self.state += 1;
        // Show the lights
        self.update(lights);
        // Wait
        Duration::Millis(400)
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        // Show the lights
        let mut deg = (self.state * 137) % 360;
        for i in 0..lights.len() {
            deg = (deg + 137) % 360;
            lights[i] = ColorLab {
                l: self.brightness,
                a: sin(deg, self.radius as isize) as i8,
                b: cos(deg, self.radius as isize) as i8,
            }
            .to_srgb_clamped();
        }
    }
}
