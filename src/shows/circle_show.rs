use crate::color::*;
use crate::int_math::{cos, inc, sin};
use crate::lights::*;
use crate::rng::Rng;
use core::cmp;

// Number of lights to store in state. In there are more lights in the actual
// strip, cycle these.
const SIZE: usize = 64;

// How long to wait between light updates, in ms.
const DURATION: u32 = 50;

// How much the _hue change velocity_ of each light changes over time, as a max
// number of degrees per DURATION per DURATION. (The hues' velocities take a
// random walk.)
const HUE_CHANGE_RATE: i32 = 3;

/// A demo lightshow with lights of randomly varying hue, that all average to a
/// controllable center color.
pub struct CircleShow {
    center: ColorLab,          // the average color
    radius: i32,               // cached max lab radius
    rng: Rng,                  // a randomish number generator
    state: [(i32, i32); SIZE], // (hue angle in degrees/step, velocity in degrees)
}

impl CircleShow {
    pub fn preset_bright(&mut self) {
        self.center = ColorLab { l: 70, a: 0, b: 0 };
        self.calculate_radius();
    }

    pub fn preset_red(&mut self) {
        self.center = ColorLab { l: 50, a: 35, b: 5 };
        self.calculate_radius();
    }

    pub fn preset_yellow(&mut self) {
        self.center = ColorLab { l: 70, a: 0, b: 30 };
        self.calculate_radius();
    }

    pub fn change_red(&mut self, delta: i8) {
        inc(&mut self.center.a, delta, -60, 60);
        self.calculate_radius();
    }

    pub fn change_yellow(&mut self, delta: i8) {
        inc(&mut self.center.b, delta, -60, 60);
        self.calculate_radius();
    }

    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.center.l, delta, 0, 99);
        self.calculate_radius();
    }

    fn calculate_radius(&mut self) {
        self.radius = self.center.max_radius() as i32;
    }
}

impl LightShow for CircleShow {
    fn new() -> CircleShow {
        let mut rng = Rng::new(161051);
        let mut state = [(0, 0); SIZE];
        let var = HUE_CHANGE_RATE;
        for (ref mut vel, ref mut pos) in state.iter_mut() {
            *vel = rng.next_in_range(0, 360);
            *pos = rng.next_in_range(-var, var + 1);
        }
        let mut show = CircleShow {
            radius: 0,
            center: ColorLab { l: 70, a: 0, b: 0 },
            rng,
            state,
        };
        show.calculate_radius();
        show
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state (random velocity walk on hue circles)
        for i in 0..SIZE {
            let var = HUE_CHANGE_RATE;
            let delta_velocity = self.rng.next_in_range(-1, 2)
                * self.rng.next_in_range(-1, 2)
                * self.rng.next_in_range(-1, 2);
            let new_velocity = self.state[i].1 + delta_velocity;
            self.state[i].1 = cmp::min(cmp::max(new_velocity, -var), var + 1);
            self.state[i].0 += self.state[i].1;
        }
        // Show the lights (cycle as needed)
        self.update(lights);
        // Wait
        Duration::Millis(DURATION)
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        let center = self.center;
        for (i, light) in lights.iter_mut().enumerate() {
            let deg = self.state[i % SIZE].0;
            let a = (sin(deg, self.radius) as i8).wrapping_add(center.a);
            let b = (cos(deg, self.radius) as i8).wrapping_add(center.b);
            let color = ColorLab { l: center.l, a, b }.to_srgb_clamped();
            *light = color;
        }
    }
}
