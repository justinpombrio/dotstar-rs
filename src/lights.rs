use crate::color::ColorRgb;

pub enum Timeout {
    Millis(usize),
    Never,
}

pub trait LightShow<Settings> {
    fn next(&mut self, lights: &mut [ColorRgb]) -> Timeout;
    fn update_settings(&mut self, settings: &Settings);
}

pub trait LightStrip {
    type Error;
    fn show(&mut self, lights: &[ColorRgb]) -> Result<(), Self::Error>;
}
