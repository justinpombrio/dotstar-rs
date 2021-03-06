use crate::color::ColorRgb;

pub enum Duration {
    Millis(u32),
    Forever,
}

pub trait LightShow {
    fn new() -> Self;
    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration;
    fn update(&mut self, lights: &mut [ColorRgb]);
}

pub trait LightStrip {
    type Error;
    fn show(&mut self, lights: &[ColorRgb]) -> Result<(), Self::Error>;
}

impl Duration {
    pub fn subtract(&mut self, millis: u32) {
        if let Duration::Millis(ref mut ms) = self {
            *ms = ms.saturating_sub(millis)
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Duration::Millis(ms) => *ms == 0,
            Duration::Forever => false,
        }
    }
}
