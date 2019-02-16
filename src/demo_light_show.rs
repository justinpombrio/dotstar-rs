use crate::color::*;
use crate::lights::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// Number of lights to store in state. In there are more lights in the actual
// strip, cycle these.
const SIZE: usize = 64;

// How long to wait between light updates, in ms.
const DURATION: u32 = 10;

// How quickly to vary hue.
const VARIATION: isize = 2;

/// Controls the brightness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoSettings {
    pub brightness: i8,
}

impl DemoSettings {
    /// Make it 10% brighter.
    pub fn inc(&mut self) {
        if self.brightness < 100 {
            self.brightness += 10;
        }
    }

    /// Make it 10% dimmer.
    pub fn dec(&mut self) {
        if self.brightness >= 10 {
            self.brightness -= 10;
        }
    }
}

impl Default for DemoSettings {
    fn default() -> DemoSettings {
        DemoSettings { brightness: 70 }
    }
}

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct Demo {
    settings: DemoSettings,
    rng: StdRng,
    state: [isize; SIZE], // hue angle in degrees
}

impl LightShow for Demo {
    type Settings = DemoSettings;

    fn new(settings: &DemoSettings) -> Demo {
        let mut rng = StdRng::seed_from_u64(161051);
        let mut state = [0; SIZE];
        for i in 0..SIZE {
            state[i] = rng.gen_range(0, 360);
        }
        Demo {
            settings: *settings,
            rng: rng,
            state: state,
        }
    }

    fn update_settings(&mut self, settings: &DemoSettings) {
        self.settings = *settings;
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state (random walk on hue circles)
        for i in 0..SIZE {
            self.state[i] += self.rng.gen_range(-VARIATION, VARIATION);
        }
        // Show the lights (cycle if needed)
        for i in 0..lights.len() {
            let l = self.settings.brightness;
            let deg = self.state[i % SIZE];
            let radius = lab_radius(l).unwrap_or(0) as isize;
            let a = sin(deg, radius) as i8;
            let b = cos(deg, radius) as i8;
            let color = ColorLab { l, a, b }.to_srgb_clamped();
            lights[i] = color;
        }
        // Wait
        Duration::Millis(DURATION)
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
