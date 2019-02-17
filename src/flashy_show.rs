use crate::color::*;
use crate::lights::*;

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct FlashyShow {
    state: isize,
}

impl LightShow for FlashyShow {
    type Settings = ();

    fn new(_: &()) -> FlashyShow {
        FlashyShow { state: 0 }
    }

    fn update_settings(&mut self, _: &()) {}

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state
        self.state += 1;
        let mut deg = (self.state * 137) % 360;
        // Show the lights
        for i in 0..lights.len() {
            deg = (deg + 137) % 360;
            lights[i] = ColorLab {
                l: 70,
                a: sin(deg, 40) as i8,
                b: cos(deg, 40) as i8,
            }
            .to_srgb_clamped();
        }
        // Wait
        Duration::Millis(400)
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
