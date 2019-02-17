use crate::color::*;
use crate::lights::*;
use crate::rng::Rng;

// Number of lights to store in state. In there are more lights in the actual
// strip, cycle these.
const SIZE: usize = 64;

// How long to wait between light updates, in ms.
const DURATION: u32 = 10;

/// Settings for this demo light show.
///
/// - **Center_color:** The average color of the lights.
/// - **Hue_change_rate:** How much the hue of the lights changes over time, as
///   a max number of degrees per 100ms. (The hues take a random walk.)
/// - **Color_variation:** How much the color varies between the lights, as a
///   percent of how high it could be at max.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircleShowSettings {
    pub center_color: ColorLab,
    pub hue_change_rate: i8,
    pub color_variation: i8,
}

impl Default for CircleShowSettings {
    fn default() -> CircleShowSettings {
        CircleShowSettings {
            center_color: ColorLab { l: 70, a: 0, b: 0 },
            hue_change_rate: 2,
            color_variation: 100,
        }
    }
}

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct CircleShow {
    settings: CircleShowSettings,
    rng: Rng,
    state: [isize; SIZE], // hue angle in degrees
}

impl LightShow for CircleShow {
    type Settings = CircleShowSettings;

    fn new(settings: &CircleShowSettings) -> CircleShow {
        let mut rng = Rng::new(161051);
        let mut state = [0; SIZE];
        for i in 0..SIZE {
            state[i] = rng.next_in_range(0, 360) as isize;
        }
        CircleShow {
            settings: *settings,
            rng: rng,
            state: state,
        }
    }

    fn update_settings(&mut self, settings: &CircleShowSettings) {
        self.settings = *settings;
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state (random walk on hue circles)
        for i in 0..SIZE {
            let var = self.settings.hue_change_rate;
            self.state[i] +=
                self.rng.next_in_range(-var as i32, (var + 1) as i32) as isize;
        }
        // Show the lights (cycle as needed)
        for i in 0..lights.len() {
            let deg = self.state[i % SIZE];
            let center = self.settings.center_color;
            let colorfulness = self.settings.color_variation;
            let max_radius = center.max_radius() as isize;
            let radius = colorfulness as isize * max_radius / 100;
            let a = sin(deg, radius) as i8 + center.a;
            let b = cos(deg, radius) as i8 + center.b;
            let color = ColorLab { l: center.l, a, b }.to_srgb_clamped();
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
