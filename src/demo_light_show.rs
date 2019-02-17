use crate::color::*;
use crate::lights::*;
use crate::rng::Rng;

// Number of lights to store in state. In there are more lights in the actual
// strip, cycle these.
const SIZE: usize = 64;

// How long to wait between light updates, in ms.
const DURATION: u32 = 10;

/// Controls the brightness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoSettings {
    pub brightness: i8,
    pub variation: i8,
    pub saturation: i8
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

    /// Make it vary faster.
    pub fn inc_variation(&mut self) {
        if self.variation < 100 {
            self.variation += 1;
        }
    }

    /// Make it vary slower.
    pub fn dec_variation(&mut self) {
        if self.variation >= 1 {
            self.variation -= 1;
        }
    }

    /// Make it more saturated.
    pub fn inc_saturation(&mut self) {
        if self.saturation < 100 {
            self.saturation += 10;
        }
    }
    
    /// Make it less saturated.
    pub fn dec_saturation(&mut self) {
        if self.saturation >= 10 {
            self.saturation -= 10;
        }
    }
}

impl Default for DemoSettings {
    fn default() -> DemoSettings {
        DemoSettings {
            brightness: 70,
            variation: 2,
            saturation: 100
        }
    }
}

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct Demo {
    settings: DemoSettings,
    rng: Rng,
    state: [isize; SIZE], // hue angle in degrees
}

impl LightShow for Demo {
    type Settings = DemoSettings;

    fn new(settings: &DemoSettings) -> Demo {
        let mut rng = Rng::new(161051);
        let mut state = [0; SIZE];
        for i in 0..SIZE {
            state[i] = rng.next_in_range(0, 360) as isize;
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
            let var = self.settings.variation;
            self.state[i] += self.rng.next_in_range(-var as i32, (var + 1) as i32) as isize;
        }
        // Show the lights (cycle if needed)
        for i in 0..lights.len() {
            let l = self.settings.brightness;
            let deg = self.state[i % SIZE];
            let max_radius = lab_radius(l).unwrap_or(0);
            let radius = (self.settings.saturation as i32 * max_radius as i32 / 100) as isize;
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
