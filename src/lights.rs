use crate::color::ColorRgb;

pub enum Duration {
    Millis(usize),
    Forever,
}

pub trait LightShow {
    type Settings;

    fn new(settings: &Self::Settings) -> Self;
    fn next(&mut self, lights: &mut [ColorRgb]) -> Duration;
    fn update_settings(&mut self, settings: &Self::Settings);
}

pub trait LightStrip {
    type Error;
    fn show(&mut self, lights: &[ColorRgb]) -> Result<(), Self::Error>;
}
