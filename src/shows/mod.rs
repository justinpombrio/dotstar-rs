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

use self::KnobEvent::*;
use self::Mode::*;

/// A standard set of light shows. You may also make your own, or use the light
/// shows individually.
pub struct DemoLightShows {
    mode: Mode,
    solid_show: SolidShow,
    circle_show: CircleShow,
    wave_show: WaveShow,
    strobe_show: StrobeShow,
}

enum Mode {
    OffMode,
    SolidMode,
    CircleMode,
    WaveMode,
    StrobeMode,
}

pub enum KnobEvent {
    Button,
    Left,
    Right,
}

impl DemoLightShows {
    pub fn new() -> DemoLightShows {
        DemoLightShows {
            mode: SolidMode,
            solid_show: SolidShow::new(),
            circle_show: CircleShow::new(),
            wave_show: WaveShow::new(),
            strobe_show: StrobeShow::new(),
        }
    }

    pub fn prev_mode(&mut self) {
        match self.mode {
            OffMode => self.mode = StrobeMode,
            SolidMode => self.mode = OffMode,
            CircleMode => self.mode = SolidMode,
            WaveMode => self.mode = CircleMode,
            StrobeMode => self.mode = WaveMode,
        }
    }

    pub fn next_mode(&mut self) {
        match self.mode {
            OffMode => self.mode = SolidMode,
            SolidMode => self.mode = CircleMode,
            CircleMode => self.mode = WaveMode,
            WaveMode => self.mode = StrobeMode,
            StrobeMode => self.mode = OffMode,
        }
    }

    pub fn knob_event(
        &mut self,
        lights: &mut [ColorRgb],
        which_knob: usize,
        event: KnobEvent,
    ) {
        match self.mode {
            OffMode => (),
            SolidMode => {
                let show = &mut self.solid_show;
                match (which_knob, event) {
                    (0, Left) => show.change_brightness(-10),
                    (0, Right) => show.change_brightness(10),
                    (1, Left) => show.change_red(-10),
                    (1, Right) => show.change_red(10),
                    (2, Left) => show.change_yellow(-10),
                    (2, Right) => show.change_yellow(10),
                    (0, Button) => show.preset_bright(),
                    (1, Button) => show.preset_red(),
                    (2, Button) => show.preset_yellow(),
                    _ => panic!("Invalid event"),
                }
            }
            CircleMode => {
                let show = &mut self.circle_show;
                match (which_knob, event) {
                    (0, Left) => show.change_brightness(-10),
                    (0, Right) => show.change_brightness(10),
                    (1, Left) => show.change_red(-10),
                    (1, Right) => show.change_red(10),
                    (2, Left) => show.change_yellow(-10),
                    (2, Right) => show.change_yellow(10),
                    (0, Button) => show.preset_bright(),
                    (1, Button) => show.preset_red(),
                    (2, Button) => show.preset_yellow(),
                    _ => panic!("Invalid event"),
                }
            }
            WaveMode => {
                let show = &mut self.wave_show;
                match (which_knob, event) {
                    (0, Left) => show.change_brightness(-10),
                    (0, Right) => show.change_brightness(10),
                    (1, Left) => show.change_delay(-50),
                    (1, Right) => show.change_delay(50),
                    (2, Left) => show.change_curvature(-1),
                    (2, Right) => show.change_curvature(1),
                    (0, Button) => show.preset_slow(),
                    (1, Button) => show.preset_fast(),
                    (2, Button) => show.preset_golden(),
                    _ => panic!("Invalid event"),
                }
            }
            StrobeMode => {
                let show = &mut self.strobe_show;
                match (which_knob, event) {
                    (0, Left) => show.change_brightness(-10),
                    (0, Right) => show.change_brightness(10),
                    (1, Left) => show.change_hue(-10),
                    (1, Right) => show.change_hue(10),
                    (2, Left) => show.change_delay(-5),
                    (2, Right) => show.change_delay(5),
                    (0, Button) => show.preset1(),
                    (1, Button) => show.preset2(),
                    (2, Button) => show.preset3(),
                    _ => panic!("Invalid event"),
                }
            }
        }
        self.update(lights);
    }

    pub fn next_lights(&mut self, lights: &mut [ColorRgb]) -> Duration {
        match self.mode {
            OffMode => {
                for light in lights {
                    *light = ColorRgb { r: 0, g: 0, b: 0 };
                }
                Duration::Forever
            }
            SolidMode => self.solid_show.next(lights),
            CircleMode => self.circle_show.next(lights),
            WaveMode => self.wave_show.next(lights),
            StrobeMode => self.strobe_show.next(lights),
        }
    }

    pub fn update(&mut self, lights: &mut [ColorRgb]) {
        match self.mode {
            OffMode => {
                for light in lights {
                    *light = ColorRgb { r: 0, g: 0, b: 0 };
                }
            }
            SolidMode => self.solid_show.update(lights),
            CircleMode => self.circle_show.update(lights),
            WaveMode => self.wave_show.update(lights),
            StrobeMode => self.strobe_show.update(lights),
        }
    }
}
