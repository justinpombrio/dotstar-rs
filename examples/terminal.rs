use dotstar::{Demo, Light, LightShow, LightStrip, Lights};

use core::time;

use std::io;
use std::io::Write;
use std::thread;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, input, raw, screen, style};

pub const DELAY: time::Duration = time::Duration::from_millis(500);

fn main() {
    let demo = Demo::new();
    let mut renderer = TerminalRenderer::new();
    renderer.show(demo);
}

pub struct TerminalRenderer {
    stdin: input::Keys<termion::AsyncReader>,
    stdout: screen::AlternateScreen<raw::RawTerminal<io::Stdout>>,
}

impl LightStrip for TerminalRenderer {
    fn show<S: LightShow>(&mut self, light_show: S) {
        match self.run_or_err(light_show) {
            Ok(()) => (),
            Err(msg) => panic!("Failed to render light show! {}", msg),
        }
        let _ = write!(self.stdout, "{}", cursor::Show);
    }
}

impl TerminalRenderer {
    pub fn new() -> TerminalRenderer {
        let stdin = termion::async_stdin().keys();
        let stdout = screen::AlternateScreen::from(io::stdout().into_raw_mode().unwrap());
        TerminalRenderer {
            stdin: stdin,
            stdout: stdout,
        }
    }

    fn run_or_err<S: LightShow>(&mut self, mut light_show: S) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::Hide)?;
        loop {
            let lights = light_show.next();
            // write!(self.stdout, "{}", lights)?;
            write_lights(&mut self.stdout, &lights)?;
            self.stdout.flush()?;
            thread::sleep(DELAY);
            for key in &mut self.stdin {
                match key? {
                    Key::Esc | Key::Char('q') | Key::Ctrl('c') => return Ok(()),
                    _ => (),
                }
            }
        }
    }
}

fn write_light<W>(f: &mut W, light: &Light) -> io::Result<()>
where
    W: Write,
{
    let color = color::Rgb(light.color.0, light.color.1, light.color.2);
    write!(f, "{}", color::Fg(color))?;
    write!(f, "⬤ ")
}

fn write_lights<W>(f: &mut W, all_lights: &Lights) -> io::Result<()>
where
    W: Write,
{
    write!(f, "{}{}", clear::All, cursor::Goto(3, 2))?;
    for (i, light) in all_lights.lights.iter().enumerate() {
        if i != 0 {
            write!(f, "{}", style::Reset)?;
            write!(f, "-")?;
        }
        write_light(f, light)?;
    }
    Ok(())
}
