use crate::{Light, Lights};

use embedded_hal::blocking::spi;

const START_FRAME: [u8; 4] = [0, 0, 0, 0];

pub struct DotstarStrip<SPI: spi::Write<u8>> {
    spi_bus: SPI,
}

impl<SPI: spi::Write<u8>> DotstarStrip<SPI> {
    pub fn new(spi_bus: SPI) -> Self {
        Self { spi_bus }
    }

    pub fn send(&mut self, lights: &Lights) -> Result<(), SPI::Error> {
        // TODO combine into fewer write calls? does it matter?
        self.spi_bus.write(&START_FRAME)?;
        for light in &lights.lights {
            self.spi_bus.write(&led_frame(light))?;
        }

        // End frame:
        for _ in 0..ceiling(lights.lights.len(), 8) {
            self.spi_bus.write(&[0])?;
        }
        Ok(())
    }
}

fn led_frame(light: &Light) -> [u8; 4] {
    let prefix_and_global_brightness = 255;
    [
        prefix_and_global_brightness,
        light.color.blue,
        light.color.green,
        light.color.red,
    ]
}

fn ceiling(num: usize, divisor: usize) -> usize {
    // Ceiling of integer division
    (num + divisor - 1) / divisor
}
