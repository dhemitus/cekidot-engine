use anyhow::Result;
use cekidot::engine::{render_loop::LoopState, state::Game};
use std::time::Duration;

fn main() {
    let mut game = Game::new();

    pollster::block_on(cekidot::run(
        120,
        640,
        480,
        "glfw loop",
        &mut game,
        |s| -> Result<LoopState> {
            s.update_called += 1;
            Ok(LoopState::Continue)
        },
        |s, d| -> Result<LoopState> {
            s.render_called += 1;
            s.time_passed += d;

            if s.time_passed > Duration::from_secs(1) {
                println!("update fps: {:.2}", s.update_called as f64 / 1f64);
                println!("render fps: {:.2}", s.render_called as f64 / 1f64);

                s.update_called = 0;
                s.render_called = 0;
                s.time_passed = Duration::default();
            }

            std::thread::sleep(Duration::from_millis(4));

            Ok(LoopState::Continue)
        },
    ))
}
