use std::{fmt, io};
use std::io::Write;
use std::thread;

use termion;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::{clear, style, cursor, color, input, screen, raw};
use termion::event::Key;

use crate::lights::*;


impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = color::Rgb(self.color.0, self.color.1, self.color.2);
        write!(f, "{}", color::Fg(color))?;
        write!(f, "⬤ ")
    }    
}

impl fmt::Display for Lights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", clear::All, cursor::Goto(3, 2))?;
        for (i, light) in self.lights.iter().enumerate() {
            if i != 0 {
                write!(f, "{}", style::Reset)?;
                write!(f, "-")?;
            }
            write!(f, "{}", light)?;
        }
        Ok(())
    }
}

pub struct TerminalRenderer {
    stdin: input::Keys<termion::AsyncReader>,
    stdout: screen::AlternateScreen<raw::RawTerminal<io::Stdout>>
}

impl LightStrip for TerminalRenderer {
    fn show<S: LightShow>(&mut self, light_show: S) {
        match self.run_or_err(light_show) {
            Ok(()) => (),
            Err(msg) => panic!("Failed to render light show! {}", msg)
        }
        let _ = write!(self.stdout, "{}", cursor::Show);
    }
}

impl TerminalRenderer {
    pub fn new() -> TerminalRenderer {
        let stdin = termion::async_stdin().keys();
        let stdout = screen::AlternateScreen::from(
            io::stdout().into_raw_mode().unwrap());
        TerminalRenderer {
            stdin: stdin,
            stdout: stdout
        }
    }

    fn run_or_err<S: LightShow>(&mut self, mut light_show: S) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::Hide)?;
        loop {
            let lights = light_show.next();
            write!(self.stdout, "{}", lights)?;
            self.stdout.flush()?;
            thread::sleep(DELAY);
            for key in &mut self.stdin {
                match key? {
                    Key::Esc | Key::Char('q') | Key::Ctrl('c') => return Ok(()),
                    _ => ()
                }
            }
        }
    }
}
