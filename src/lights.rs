pub const NUM_LIGHTS: usize = 5;

#[derive(Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

pub enum Timeout {
    Millis(usize),
    Never,
}

pub trait LightShow<Settings> {
    fn next(&mut self, lights: &mut [Rgb]) -> Timeout;
    fn update_settings(&mut self, settings: &Settings);
}

pub trait LightStrip<Settings> {
    fn show<S: LightShow<Settings>>(&mut self, light_show: S);
}
