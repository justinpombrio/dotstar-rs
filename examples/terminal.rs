use dotstar::{ColorRgb, Demo, DemoSettings, Duration, LightShow, LightStrip};

use core::time;

use std::io;
use std::io::Write;
use std::thread;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, input, raw, screen, style};

fn main() {
    let mut settings = DemoSettings::new();
    let mut demo = Demo::new(&settings);
    let mut renderer = TerminalRenderer::new();
    let mut lights = [ColorRgb { r: 0, g: 0, b: 0 }; 20];
    let mut never = false;
    'outer: loop {
        if never {
            thread::sleep(time::Duration::from_millis(10));
        } else {
            let duration = demo.next(&mut lights);
            match renderer.show(&lights) {
                Ok(()) => (),
                Err(msg) => panic!("Failed to render light show! {}", msg),
            }
            match duration {
                Duration::Millis(ms) => {
                    thread::sleep(time::Duration::from_millis(ms as u64))
                }
                Duration::Forever => {
                    never = true;
                }
            }
        }
        for key in &mut renderer.stdin {
            match key.expect("Could not read key") {
                Key::Up => {
                    settings.inc();
                    demo.update_settings(&settings);
                }
                Key::Down => {
                    settings.dec();
                    demo.update_settings(&settings);
                }
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => break 'outer,
                _ => continue,
            }
        }
    }
}

pub struct TerminalRenderer {
    pub stdin: input::Keys<termion::AsyncReader>,
    stdout: screen::AlternateScreen<raw::RawTerminal<io::Stdout>>,
}

impl LightStrip for TerminalRenderer {
    type Error = io::Error;

    fn show(&mut self, lights: &[ColorRgb]) -> io::Result<()> {
        write_lights(&mut self.stdout, &lights)?;
        self.stdout.flush()
    }
}

impl TerminalRenderer {
    pub fn new() -> TerminalRenderer {
        let stdin = termion::async_stdin().keys();
        let mut stdout = screen::AlternateScreen::from(
            io::stdout().into_raw_mode().unwrap(),
        );
        write!(stdout, "{}", cursor::Hide).expect("Could not hide cursor");
        TerminalRenderer {
            stdin: stdin,
            stdout: stdout,
        }
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = write!(self.stdout, "{}", cursor::Show);
    }
}

fn write_light<W>(f: &mut W, light: &ColorRgb) -> io::Result<()>
where
    W: Write,
{
    let tcolor = color::Rgb(light.r, light.g, light.b);
    write!(f, "{}", color::Fg(tcolor))?;
    write!(f, "⬤ ")
}

fn write_lights<W>(f: &mut W, all_lights: &[ColorRgb]) -> io::Result<()>
where
    W: Write,
{
    write!(f, "{}{}", clear::All, cursor::Goto(3, 2))?;
    for (i, light) in all_lights.iter().enumerate() {
        if i != 0 {
            write!(f, "{}", style::Reset)?;
            write!(f, "-")?;
        }
        write_light(f, light)?;
    }
    Ok(())
}
