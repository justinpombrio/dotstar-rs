use dotstar::{Demo, TerminalRenderer, LightShow, LightStrip};

fn main() {
    let demo = Demo::new();
    let mut renderer = TerminalRenderer::new();
    renderer.show(demo);
}
