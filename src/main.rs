use anyhow::Result;
use cekidot::engine::{
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

fn update(g: &mut GameOn, i: &mut GlfwInputState) -> Result<LoopState> {
    g.update_called += 1;

    if i.is_key_down(KeyboardKey::A) {
        println!("downed {:?} {}", KeyboardKey::A, g.adding);
        g.adding += 1;
    }

    Ok(LoopState::Continue)
}

fn render(g: &mut GameOn, i: &mut GlfwInputState, d: Duration) -> Result<LoopState> {
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

    let mut input = GlfwInputState::new();

    pollster::block_on(cekidot::run(
        120,
        1024,
        768,
        "glfw loop",
        &mut game,
        &mut input,
        update,
        render,
    ))
}
