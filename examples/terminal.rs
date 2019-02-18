use dotstar::{ColorRgb, DemoLightShows, Duration, KnobEvent, LightStrip};

use core::time;

use std::io;
use std::io::Write;
use std::thread;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, input, raw, screen, style};

use KnobEvent::{Button, Left, Right};

fn main() {
    let mut shows = DemoLightShows::new();
    let mut renderer = TerminalRenderer::new();
    let mut lights = [ColorRgb { r: 0, g: 0, b: 0 }; 50];
    let mut forever = false;
    'outer: loop {
        if forever {
            thread::sleep(time::Duration::from_millis(10));
        } else {
            let duration = shows.next_lights(&mut lights);
            match renderer.show(&lights) {
                Ok(()) => (),
                Err(msg) => panic!("Failed to render light show! {}", msg),
            }
            match duration {
                Duration::Millis(ms) => {
                    thread::sleep(time::Duration::from_millis(ms as u64))
                }
                Duration::Forever => {
                    forever = true;
                }
            }
        }
        for key in &mut renderer.stdin {
            match key.expect("Could not read key") {
                Key::Char('m') => shows.next_mode(),
                Key::Char('1') => shows.knob_event(&mut lights, 0, Button),
                Key::Char('2') => shows.knob_event(&mut lights, 1, Button),
                Key::Char('3') => shows.knob_event(&mut lights, 2, Button),
                Key::Down => shows.knob_event(&mut lights, 0, Left),
                Key::Up => shows.knob_event(&mut lights, 0, Right),
                Key::Left => shows.knob_event(&mut lights, 1, Left),
                Key::Right => shows.knob_event(&mut lights, 1, Right),
                Key::Char('[') => shows.knob_event(&mut lights, 2, Left),
                Key::Char(']') => shows.knob_event(&mut lights, 2, Right),
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
        write_instructions(&mut self.stdout)?;
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

fn write_instructions<W>(f: &mut W) -> io::Result<()>
where
    W: Write,
{
    for (i, (r, g, b, msg)) in LINES.iter().enumerate() {
        let line_num = (i + 8) as u16;
        write!(f, "{}", cursor::Goto(5, line_num))?;
        write!(f, "{}", color::Fg(color::Rgb(*r, *g, *b)))?;
        write!(f, "{}", msg)?;
    }
    Ok(())
}

static LINES: [(u8, u8, u8, &'static str); 14] = [
    (255, 255, 255, "m: switch mode"),
    (255, 255, 255, "q,Esc: quit"),
    (0, 0, 0, ""),
    (255, 255, 255, "↑: knob 1 left"),
    (255, 255, 255, "↓: knob 1 right"),
    (255, 255, 255, "1: knob 1 button"),
    (0, 0, 0, ""),
    (255, 255, 255, "←: knob 2 right"),
    (255, 255, 255, "→: knob 2 left"),
    (255, 255, 255, "2: knob 2 button"),
    (0, 0, 0, ""),
    (255, 255, 255, "[: knob 3 left"),
    (255, 255, 255, "]: knob 3 right"),
    (255, 255, 255, "3: knob 3 button"),
];
