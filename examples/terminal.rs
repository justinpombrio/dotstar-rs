use dotstar::{
    CircleShow, ColorRgb, Duration, FlashyShow, LightShow, LightStrip,
};

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
    let mut circle_show = CircleShow::new();
    let mut flashy_show = FlashyShow::new();
    let mut renderer = TerminalRenderer::new();
    let mut lights = [ColorRgb { r: 0, g: 0, b: 0 }; 20];
    let mut forever = false;
    'outer: loop {
        if forever {
            thread::sleep(time::Duration::from_millis(10));
        } else {
            let duration = if renderer.mode {
                circle_show.next(&mut lights)
            } else {
                flashy_show.next(&mut lights)
            };
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
                Key::Char('m') => renderer.mode = !renderer.mode,
                Key::Char('r') => circle_show.change_red(5),
                Key::Char('g') => circle_show.change_red(-5),
                Key::Char('y') => circle_show.change_yellow(5),
                Key::Char('b') => circle_show.change_yellow(-5),
                Key::Up => circle_show.change_brightness(10),
                Key::Down => circle_show.change_brightness(-10),
                Key::Right => circle_show.change_color_variation(10),
                Key::Left => circle_show.change_color_variation(-10),
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => break 'outer,
                _ => continue,
            }
        }
    }
}

pub struct TerminalRenderer {
    pub stdin: input::Keys<termion::AsyncReader>,
    stdout: screen::AlternateScreen<raw::RawTerminal<io::Stdout>>,
    mode: bool,
}

impl LightStrip for TerminalRenderer {
    type Error = io::Error;

    fn show(&mut self, lights: &[ColorRgb]) -> io::Result<()> {
        write_lights(&mut self.stdout, &lights)?;
        write_instructions(&mut self.stdout, self.mode)?;
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
            mode: true,
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

fn write_instructions<W>(f: &mut W, mode: bool) -> io::Result<()>
where
    W: Write,
{
    write!(f, "{}", color::Fg(color::Rgb(255, 255, 150)))?;
    let mode_name = if mode { "circle show" } else { "flashy show" };
    write!(f, "{}", cursor::Goto(5, 5))?;
    write!(f, "Current mode: {}", mode_name)?;
    write!(f, "{}", cursor::Goto(5, 6))?;
    write!(f, "------------------------")?;
    for (i, (r, g, b, msg)) in LINES.iter().enumerate() {
        let line_num = (i + 8) as u16;
        write!(f, "{}", cursor::Goto(5, line_num))?;
        write!(f, "{}", color::Fg(color::Rgb(*r, *g, *b)))?;
        write!(f, "{}", msg)?;
    }
    Ok(())
}

static LINES: [(u8, u8, u8, &'static str); 14] = [
    (255, 255, 255, "m: switch mode (circle vs. flashy)"),
    (255, 255, 255, "q,Esc: quit"),
    (0, 0, 0, ""),
    (255, 255, 150, "Settings for circle mode"),
    (255, 255, 150, "------------------------"),
    (0, 0, 0, ""),
    (255, 255, 255, "→: increase color variation"),
    (255, 255, 255, "←: decrease color variation"),
    (255, 255, 255, "↑: increase brightness"),
    (255, 255, 255, "↓: decrease brightness"),
    (255, 255, 255, "r: more red"),
    (255, 255, 255, "g: more green"),
    (255, 255, 255, "b: more blue"),
    (255, 255, 255, "y: more yellow"),
];
