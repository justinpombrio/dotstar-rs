use crate::color::*;
use crate::int_math::{cos, inc, inc_i32, sin};
use crate::lights::*;

/// A demo lightshow with lights whose hues cycle in a wave. The settings can
/// control the speed of the wave (how frequently the lights move down the
/// strip), and the curvature of the wave (how many degrees the hue varies
/// between adjacent lights).
pub struct WaveShow {
    center: ColorLab, // average color
    delay: i32,       // delay between updates
    curvature: i32,   // number of hue degrees between adjacent lights
    state: isize,     // counts how many updates there have been
    radius: i8,       // cached max lab radius
}

impl WaveShow {
    pub fn preset_slow(&mut self) {
        self.center = ColorLab { l: 70, a: 0, b: 0 };
        self.radius = self.center.max_radius();
        self.delay = 50;
        self.curvature = 1;
    }

    pub fn preset_fast(&mut self) {
        self.center = ColorLab { l: 70, a: 0, b: 10 };
        self.radius = self.center.max_radius();
        self.delay = 50;
        self.curvature = 17;
    }

    pub fn preset_golden(&mut self) {
        self.center = ColorLab { l: 70, a: 0, b: 10 };
        self.radius = self.center.max_radius();
        self.delay = 400;
        self.curvature = 137;
    }

    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.center.l, delta, 0, 100);
        self.radius = self.center.max_radius();
    }

    pub fn change_delay(&mut self, delta: i32) {
        inc_i32(&mut self.delay, delta, 50, 4000);
    }

    pub fn change_curvature(&mut self, delta: i32) {
        inc_i32(&mut self.curvature, delta, -180, 180);
    }
}

impl LightShow for WaveShow {
    fn name() -> &'static str {
        "Wave"
    }

    fn new() -> WaveShow {
        let mut show = WaveShow {
            center: ColorLab { l: 70, a: 0, b: 0 },
            radius: 0,
            delay: 50,
            curvature: 1,
            state: 0,
        };
        show.radius = show.center.max_radius();
        show
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        // Update state
        self.state += 1;
        // Show the lights
        self.update(lights);
        // Wait
        Duration::Millis(self.delay as u32)
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        // Show the lights
        let mut deg = (self.state * self.curvature as isize) % 360;
        for i in 0..lights.len() {
            deg = (deg + self.curvature as isize) % 360;
            let a = sin(deg, self.radius as isize);
            let b = cos(deg, self.radius as isize);
            lights[i] = ColorLab {
                l: self.center.l,
                a: self.center.a + a as i8,
                b: self.center.b + b as i8,
            }
            .to_srgb_clamped();
        }
    }
}
