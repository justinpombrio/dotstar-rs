use crate::color::ColorRgb;

pub enum Duration {
    Millis(u32),
    Forever,
}

pub trait LightShow {
    fn new() -> Self;
    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration;
}

pub trait LightStrip {
    type Error;
    fn show(&mut self, lights: &[ColorRgb]) -> Result<(), Self::Error>;
}

impl Duration {
    pub fn subtract(&mut self, millis: u32) {
        match self {
            Duration::Millis(ref mut ms) => *ms = ms.saturating_sub(millis),
            _ => (),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Duration::Millis(ms) => *ms == 0,
            Duration::Forever => false,
        }
    }
}
