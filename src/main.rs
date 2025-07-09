use anyhow::Result;
use cekidot::engine::{
    canvas::{Canvas, RenderableCanvas},
    input::{KeyboardKey, KeyboardState, glfw_input::GlfwInputState},
    render_loop::LoopState,
};
use std::time::Duration;

pub struct GameOn {
    pub update_called: usize,
    pub render_called: usize,
    pub adding: usize,
    pub time_passed: Duration,
}

impl GameOn {
    pub fn new() -> Self {
        Self {
            update_called: 0,
            render_called: 0,
            adding: 0,
            time_passed: Duration::default(),
        }
    }
}

pub struct CanvasOn {
    width: u32,
    height: u32,
}

impl CanvasOn {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Canvas for CanvasOn {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl RenderableCanvas for CanvasOn {
    fn on_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_resize(&mut self) -> Option<(u32, u32)> {
        Some((self.width, self.height))
    }

    fn on_start(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_end(&mut self) -> Result<()> {
        Ok(())
    }
}

fn update(g: &mut GameOn, i: &mut GlfwInputState, c: &mut CanvasOn) -> Result<LoopState> {
    g.update_called += 1;

    if i.is_key_down(KeyboardKey::A) {
        println!("downed {:?} {}", KeyboardKey::A, g.adding);
        g.adding += 1;
    }

    if i.is_key_pressed(KeyboardKey::UP) {
        println!("pressed {:?} {}", KeyboardKey::UP, g.adding);
        g.adding += 1;
    }

    Ok(LoopState::Continue)
}

fn render(
    g: &mut GameOn,
    i: &mut GlfwInputState,
    c: &mut CanvasOn,
    d: Duration,
) -> Result<LoopState> {
    g.render_called += 1;
    g.time_passed += d;
    /*
        if g.time_passed > Duration::from_secs(1) {
            println!("update fps: {:.2}", g.update_called as f64 / 1f64);
            println!("render fps: {:.2}", g.render_called as f64 / 1f64);

            g.update_called = 0;
            g.render_called = 0;
            g.time_passed = Duration::default();
        }

        std::thread::sleep(Duration::from_millis(4));
    */
    Ok(LoopState::Continue)
}

fn main() {
    let mut game = GameOn::new();

    let canvas = CanvasOn::new(1024, 768);

    let mut input = GlfwInputState::new();

    pollster::block_on(cekidot::run(
        120,
        1024,
        768,
        "glfw input",
        &mut game,
        &mut input,
        canvas,
        update,
        render,
    ))
}
