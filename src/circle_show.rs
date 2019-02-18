use crate::color::*;
use crate::int_math::{cos, inc, sin};
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
pub struct Settings {
    pub center_color: ColorLab,
    pub hue_change_rate: i8,
    pub color_variation: i8,
}

/// A demo lightshow with lights of randomly varying hue, and controllable
/// brightness.
pub struct CircleShow {
    max_radius: isize,
    settings: Settings,
    rng: Rng,
    state: [isize; SIZE], // hue angle in degrees
}

impl CircleShow {
    pub fn change_red(&mut self, delta: i8) {
        inc(&mut self.settings.center_color.a, delta, -40, 40);
        self.calculate_radius();
    }

    pub fn change_yellow(&mut self, delta: i8) {
        inc(&mut self.settings.center_color.b, delta, -40, 40);
        self.calculate_radius();
    }

    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.settings.center_color.l, delta, 0, 99);
        self.calculate_radius();
    }

    pub fn change_color_variation(&mut self, delta: i8) {
        inc(&mut self.settings.color_variation, delta, 0, 100);
    }

    fn calculate_radius(&mut self) {
        self.max_radius = self.settings.center_color.max_radius() as isize;
    }
}

impl LightShow for CircleShow {
    fn new() -> CircleShow {
        let mut rng = Rng::new(161051);
        let mut state = [0; SIZE];
        for i in 0..SIZE {
            state[i] = rng.next_in_range(0, 360) as isize;
        }
        let mut show = CircleShow {
            max_radius: 0,
            settings: Settings {
                center_color: ColorLab { l: 70, a: 0, b: 0 },
                hue_change_rate: 2,
                color_variation: 100,
            },
            rng: rng,
            state: state,
        };
        show.calculate_radius();
        show
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state (random walk on hue circles)
        for i in 0..SIZE {
            let var = self.settings.hue_change_rate as i32;
            self.state[i] += self.rng.next_in_range(-var, var + 1) as isize;
        }
        // Show the lights (cycle as needed)
        self.update(lights);
        // Wait
        Duration::Millis(DURATION)
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        let center = self.settings.center_color;
        let colorfulness = self.settings.color_variation;
        let radius = colorfulness as isize * self.max_radius / 100;
        for i in 0..lights.len() {
            let deg = self.state[i % SIZE];
            let a = (sin(deg, radius) as i8).wrapping_add(center.a);
            let b = (cos(deg, radius) as i8).wrapping_add(center.b);
            let color = ColorLab { l: center.l, a, b }.to_srgb_clamped();
            lights[i] = color;
        }
    }
}
