pub mod engine;

use anyhow::Result;
use glfw::fail_on_errors;
use std::time::Duration;

use crate::engine::render_loop::{LoopState, RenderLoop, WindowWrapper};
use crate::engine::state::Game;

pub async fn run(fps: usize, width: usize, height: usize, title: &str) {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            title,
            glfw::WindowMode::Windowed,
        )
        .unwrap();
    let mut game = Game::new().await;

    let window_wrapper: WindowWrapper = WindowWrapper::new(&mut glfw, &events, &mut window);

    let mut render_loop = RenderLoop::new(
        fps,
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
        window_wrapper,
    );

    render_loop.on_run();
}
