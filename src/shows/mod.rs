mod circle_show;
mod solid_show;
mod strobe_show;
mod wave_show;

use crate::color::ColorRgb;
use crate::lights::{Duration, LightShow};
pub use circle_show::CircleShow;
pub use solid_show::SolidShow;
pub use strobe_show::StrobeShow;
pub use wave_show::WaveShow;

/// A standard set of light shows. You may also make your own, or use the light
/// shows individually.
pub struct DemoLightShows {
    mode: Mode,
    solid_show: SolidShow,
    circle_show: CircleShow,
    wave_show: WaveShow,
    strobe_show: StrobeShow,
}

#[derive(PartialEq, Clone)]
enum Mode {
    Off,
    Solid,
    Circle,
    Wave,
    Strobe,
}

impl DemoLightShows {
    pub fn new() -> DemoLightShows {
        DemoLightShows {
            mode: Mode::Solid,
            solid_show: SolidShow::new(),
            circle_show: CircleShow::new(),
            wave_show: WaveShow::new(),
            strobe_show: StrobeShow::new(),
        }
    }

    pub fn set_mode(&mut self, mode_num: u8) -> bool {
        let mode = Mode::from(mode_num);
        if mode != self.mode {
            self.mode = mode;
            return true;
        }
        false
    }

    pub fn button_pressed(
        &mut self,
        lights: &mut [ColorRgb],
        which_button: usize,
    ) {
        match self.mode {
            Mode::Off => return,
            Mode::Solid => {
                let show = &mut self.solid_show;
                match which_button {
                    0 => show.preset_bright(),
                    1 => show.preset_red(),
                    2 => show.preset_yellow(),
                    _ => panic!("Invalid button"),
                }
            }
            Mode::Circle => {
                let show = &mut self.circle_show;
                match which_button {
                    0 => show.preset_bright(),
                    1 => show.preset_red(),
                    2 => show.preset_yellow(),
                    _ => panic!("Invalid button"),
                }
            }
            Mode::Wave => {
                let show = &mut self.wave_show;
                match which_button {
                    0 => show.preset_slow(),
                    1 => show.preset_fast(),
                    2 => show.preset_golden(),
                    _ => panic!("Invalid button"),
                }
            }
            Mode::Strobe => {
                let show = &mut self.strobe_show;
                match which_button {
                    0 => show.preset1(),
                    1 => show.preset2(),
                    2 => show.preset3(),
                    _ => panic!("Invalid button"),
                }
            }
        }
        self.update(lights);
    }

    pub fn knob_turned(
        &mut self,
        lights: &mut [ColorRgb],
        which_knob: usize,
        clicks: i16,
    ) {
        let clicks = clicks as i8;
        match self.mode {
            Mode::Off => return,
            Mode::Solid => {
                let show = &mut self.solid_show;
                match which_knob {
                    0 => show.change_brightness(10 * clicks as i8),
                    1 => show.change_red(10 * clicks as i8),
                    2 => show.change_yellow(10 * clicks as i8),
                    _ => panic!("Invalid knob"),
                }
            }
            Mode::Circle => {
                let show = &mut self.circle_show;
                match which_knob {
                    0 => show.change_brightness(10 * clicks as i8),
                    1 => show.change_red(10 * clicks as i8),
                    2 => show.change_yellow(10 * clicks as i8),
                    _ => panic!("Invalid knob"),
                }
            }
            Mode::Wave => {
                let show = &mut self.wave_show;
                match which_knob {
                    0 => show.change_brightness(10 * clicks),
                    1 => show.change_delay(50 * clicks as i32),
                    2 => show.change_curvature(clicks as i32),
                    _ => panic!("Invalid knob"),
                }
            }
            Mode::Strobe => {
                let show = &mut self.strobe_show;
                match which_knob {
                    0 => show.change_brightness(10),
                    1 => show.change_hue(10),
                    2 => show.change_delay(5),
                    _ => panic!("Invalid button"),
                }
            }
        }
        self.update(lights);
    }

    pub fn next_lights(&mut self, lights: &mut [ColorRgb]) -> Duration {
        match self.mode {
            Mode::Off => {
                for light in lights {
                    *light = ColorRgb { r: 0, g: 0, b: 0 };
                }
                Duration::Forever
            }
            Mode::Solid => self.solid_show.next(lights),
            Mode::Circle => self.circle_show.next(lights),
            Mode::Wave => self.wave_show.next(lights),
            Mode::Strobe => self.strobe_show.next(lights),
        }
    }

    pub fn update(&mut self, lights: &mut [ColorRgb]) {
        match self.mode {
            Mode::Off => {
                for light in lights {
                    *light = ColorRgb { r: 0, g: 0, b: 0 };
                }
            }
            Mode::Solid => self.solid_show.update(lights),
            Mode::Circle => self.circle_show.update(lights),
            Mode::Wave => self.wave_show.update(lights),
            Mode::Strobe => self.strobe_show.update(lights),
        }
    }
}

impl From<u8> for Mode {
    fn from(num: u8) -> Mode {
        match num {
            1 => Mode::Solid,
            2 => Mode::Circle,
            3 => Mode::Wave,
            4 => Mode::Strobe,
            _ => Mode::Off,
        }
    }
}

impl Default for DemoLightShows {
    fn default() -> DemoLightShows {
        DemoLightShows::new()
    }
}
