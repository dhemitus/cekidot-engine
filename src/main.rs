use anyhow::Result;
use cekidot::engine::render_loop::LoopState;
use std::time::Duration;

pub struct GameOn {
    pub update_called: usize,
    pub render_called: usize,
    pub time_passed: Duration,
}

impl GameOn {
    pub fn new() -> Self {
        Self {
            update_called: 0,
            render_called: 0,
            time_passed: Duration::default(),
        }
    }
}

fn update(g: &mut GameOn) -> Result<LoopState> {
    g.update_called += 1;
    Ok(LoopState::Continue)
}

fn render(g: &mut GameOn, d: Duration) -> Result<LoopState> {
    g.render_called += 1;
    g.time_passed += d;

    if g.time_passed > Duration::from_secs(1) {
        println!("update fps: {:.2}", g.update_called as f64 / 1f64);
        println!("render fps: {:.2}", g.render_called as f64 / 1f64);

        g.update_called = 0;
        g.render_called = 0;
        g.time_passed = Duration::default();
    }

    std::thread::sleep(Duration::from_millis(4));

    Ok(LoopState::Continue)
}

fn main() {
    let mut game = GameOn::new();

    pollster::block_on(cekidot::run(
        120,
        640,
        480,
        "glfw loop",
        &mut game,
        update,
        render,
    ))
}
