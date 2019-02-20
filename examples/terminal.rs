use dotstar::{ColorRgb, DemoLightShows, Duration, LightStrip};

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
    let mut shows = DemoLightShows::new();
    let mut renderer = TerminalRenderer::new();
    let mut lights = [ColorRgb { r: 0, g: 0, b: 0 }; 50];
    let mut duration = Duration::Millis(0);
    'outer: loop {
        match duration {
            Duration::Millis(ms) => {
                thread::sleep(time::Duration::from_millis(ms.into()));
                duration = shows.next_lights(&mut lights);
                match renderer.show(&lights) {
                    Ok(()) => (),
                    Err(msg) => panic!("Failed to render light show! {}", msg),
                }
            }
            Duration::Forever => {
                thread::sleep(time::Duration::from_millis(10));
            }
        }
        let mut updated = false;
        for key in &mut renderer.stdin {
            match key.expect("Could not read key") {
                Key::Char('1') => {
                    shows.set_mode(0);
                    duration = Duration::Millis(0);
                }
                Key::Char('2') => {
                    shows.set_mode(1);
                    duration = Duration::Millis(0);
                }
                Key::Char('3') => {
                    shows.set_mode(2);
                    duration = Duration::Millis(0);
                }
                Key::Char('4') => {
                    shows.set_mode(3);
                    duration = Duration::Millis(0);
                }
                Key::Char('5') => {
                    shows.set_mode(4);
                    duration = Duration::Millis(0);
                }
                Key::Char('6') => {
                    shows.set_mode(5);
                    duration = Duration::Millis(0);
                }
                Key::Char('7') => {
                    shows.set_mode(6);
                    duration = Duration::Millis(0);
                }
                Key::Char('8') => {
                    shows.set_mode(7);
                    duration = Duration::Millis(0);
                }
                Key::Char('w') => shows.button_pressed(&mut lights, 0),
                Key::Char('e') => shows.button_pressed(&mut lights, 1),
                Key::Char('r') => shows.button_pressed(&mut lights, 2),
                Key::Down => shows.knob_turned(&mut lights, 0, -1),
                Key::Up => shows.knob_turned(&mut lights, 0, 1),
                Key::Left => shows.knob_turned(&mut lights, 1, -1),
                Key::Right => shows.knob_turned(&mut lights, 1, 1),
                Key::Char('[') => shows.knob_turned(&mut lights, 2, -1),
                Key::Char(']') => shows.knob_turned(&mut lights, 2, 1),
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => break 'outer,
                _ => continue,
            }
            updated = true;
        }
        if updated {
            match renderer.show(&lights) {
                Ok(()) => (),
                Err(msg) => panic!("Failed to render light show! {}", msg),
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
        TerminalRenderer { stdin, stdout }
    }
}

impl Default for TerminalRenderer {
    fn default() -> TerminalRenderer {
        TerminalRenderer::new()
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = write!(self.stdout, "{}", cursor::Show);
    }
}

fn write_light<W>(f: &mut W, light: ColorRgb) -> io::Result<()>
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
        write_light(f, *light)?;
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
    (255, 255, 255, "1-8: switch mode"),
    (255, 255, 255, "q,Esc: quit"),
    (0, 0, 0, ""),
    (255, 255, 255, "↑: knob 1 left"),
    (255, 255, 255, "↓: knob 1 right"),
    (255, 255, 255, "w: knob 1 button"),
    (0, 0, 0, ""),
    (255, 255, 255, "←: knob 2 right"),
    (255, 255, 255, "→: knob 2 left"),
    (255, 255, 255, "e: knob 2 button"),
    (0, 0, 0, ""),
    (255, 255, 255, "[: knob 3 left"),
    (255, 255, 255, "]: knob 3 right"),
    (255, 255, 255, "r: knob 3 button"),
];
