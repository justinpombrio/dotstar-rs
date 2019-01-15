use std::time;


pub const DELAY: time::Duration = time::Duration::from_millis(500);
pub const NUM_LIGHTS: usize = 3;

pub struct Rgb(pub u8, pub u8, pub u8);

pub struct Light {
    pub color: Rgb
}

pub struct Lights {
    pub lights: [Light; NUM_LIGHTS]
}

pub trait LightShow {
    fn new() -> Self;
    fn next(&mut self) -> Lights;
}

pub trait LightStrip {
    fn show<S: LightShow>(&mut self, light_show: S);
}

impl Light {
    pub fn new(r: u8, g: u8, b: u8) -> Light {
        Light {
            color: Rgb(r, g, b)
        }
    }
}
