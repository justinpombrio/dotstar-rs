use crate::color::*;
use crate::int_math::inc;
use crate::lights::*;

/// A demo lightshow that simply shows a single solid color. The color can be
/// adjusted.
pub struct SolidShow {
    color: ColorLab, // average color
    radius: i8,      // cached max lab radius
}

impl SolidShow {
    pub fn preset_bright(&mut self) {
        self.color = ColorLab { l: 70, a: 0, b: 0 };
        self.radius = self.color.max_radius();
    }

    pub fn preset_red(&mut self) {
        self.color = ColorLab { l: 70, a: 30, b: 0 };
        self.radius = self.color.max_radius();
    }

    pub fn preset_yellow(&mut self) {
        self.color = ColorLab { l: 70, a: 0, b: 30 };
        self.radius = self.color.max_radius();
    }

    pub fn change_brightness(&mut self, delta: i8) {
        inc(&mut self.color.l, delta, 0, 100);
        self.radius = self.color.max_radius();
    }

    pub fn change_red(&mut self, delta: i8) {
        inc(&mut self.color.a, delta, -60, 60);
        self.radius = self.color.max_radius();
    }

    pub fn change_yellow(&mut self, delta: i8) {
        inc(&mut self.color.b, delta, -60, 60);
        self.radius = self.color.max_radius();
    }
}

impl LightShow for SolidShow {
    fn new() -> SolidShow {
        let mut show = SolidShow {
            color: ColorLab { l: 70, a: 0, b: 0 },
            radius: 0,
        };
        show.radius = show.color.max_radius();
        show
    }

    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration {
        self.update(lights);
        Duration::Forever
    }

    fn update(&mut self, lights: &mut [ColorRgb]) {
        for light in lights {
            *light = self.color.to_srgb_clamped();
        }
    }
}
