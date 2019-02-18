mod circle_show;
mod wave_show;

use self::KnobEvent::*;
use crate::color::ColorRgb;
use crate::lights::{Duration, LightShow};
pub use circle_show::CircleShow;
pub use wave_show::WaveShow;

/// A standard set of light shows. You may also make your own, or use the light
/// shows individually.
pub struct DemoLightShows {
    mode: usize,
    circle_show: CircleShow,
    wave_show: WaveShow,
}

pub enum KnobEvent {
    Button,
    Left,
    Right,
}

impl DemoLightShows {
    pub fn new() -> DemoLightShows {
        DemoLightShows {
            mode: 0,
            circle_show: CircleShow::new(),
            wave_show: WaveShow::new(),
        }
    }

    pub fn mode_name(&self) -> &'static str {
        match self.mode {
            0 => CircleShow::name(),
            1 => WaveShow::name(),
            _ => panic!("Invalid mode"),
        }
    }

    pub fn prev_mode(&mut self) {
        self.mode = (self.mode + 1) % 2
    }

    pub fn next_mode(&mut self) {
        self.mode = (self.mode + 1) % 2
    }

    pub fn knob_event(
        &mut self,
        lights: &mut [ColorRgb],
        which_knob: usize,
        event: KnobEvent,
    ) {
        match (self.mode, which_knob, event) {
            (0, 0, Left) => self.circle_show.change_brightness(-10),
            (0, 0, Right) => self.circle_show.change_brightness(10),
            (0, 1, Left) => self.circle_show.change_red(-10),
            (0, 1, Right) => self.circle_show.change_red(10),
            (0, 2, Left) => self.circle_show.change_yellow(-10),
            (0, 2, Right) => self.circle_show.change_yellow(10),
            (0, 0, Button) => self.circle_show.preset_bright(),
            (0, 1, Button) => self.circle_show.preset_red(),
            (0, 2, Button) => self.circle_show.preset_yellow(),
            (1, 0, Left) => self.wave_show.change_brightness(-10),
            (1, 0, Right) => self.wave_show.change_brightness(10),
            (1, 1, Left) => self.wave_show.change_delay(-50),
            (1, 1, Right) => self.wave_show.change_delay(50),
            (1, 2, Left) => self.wave_show.change_curvature(-1),
            (1, 2, Right) => self.wave_show.change_curvature(1),
            (1, 0, Button) => self.wave_show.preset_slow(),
            (1, 1, Button) => self.wave_show.preset_fast(),
            (1, 2, Button) => self.wave_show.preset_golden(),
            _ => panic!("Invalid mode or event"),
        }
        self.update(lights);
    }

    pub fn next_lights(&mut self, lights: &mut [ColorRgb]) -> Duration {
        match self.mode {
            0 => self.circle_show.next(lights),
            1 => self.wave_show.next(lights),
            _ => panic!("Invalid mode"),
        }
    }

    pub fn update(&mut self, lights: &mut [ColorRgb]) {
        match self.mode {
            0 => self.circle_show.update(lights),
            1 => self.wave_show.update(lights),
            _ => panic!("Invalid mode"),
        }
    }
}
