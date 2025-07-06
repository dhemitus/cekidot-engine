use crate::Game;
use anyhow::{Context, Result};
use glfw::{Context as OtherContext, Glfw, GlfwReceiver, Window, WindowEvent};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopState {
    Continue,
    Exit(i32),
}

pub type UpdateFn = fn(&mut Game) -> Result<LoopState>;

pub type RenderFn = fn(&mut Game, Duration) -> Result<LoopState>;

pub struct WindowWrapper<'a> {
    pub glfw: &'a mut Glfw,
    pub events: &'a GlfwReceiver<(f64, WindowEvent)>,
    pub window: &'a mut Window,
}
impl<'a> WindowWrapper<'a> {
    pub fn new(
        glfw: &'a mut Glfw,
        events: &'a GlfwReceiver<(f64, WindowEvent)>,
        window: &'a mut Window,
    ) -> Self {
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Hidden);
        window.make_current();

        Self {
            glfw,
            events,
            window,
        }
    }
}

pub struct RenderLoop<'a> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    game: &'a mut Game,
    update: UpdateFn,
    render: RenderFn,
    window_wrapper: WindowWrapper<'a>,
}

impl<'a> RenderLoop<'a> {
    pub fn new(
        fps: usize,
        game: &'a mut Game,
        update: UpdateFn,
        render: RenderFn,
        window_wrapper: WindowWrapper<'a>,
    ) -> Self {
        if fps == 0 {
            panic!("must be > 0");
        }
        Self {
            accumulator: Duration::default(),
            current_time: Instant::now(),
            last_time: Instant::now(),
            update_timestep: Duration::from_nanos((1_000_000_000f64 / fps as f64).round() as u64),
            game: game,
            update,
            render,
            window_wrapper,
        }
    }

    pub fn on_run(&mut self) {
        self.on_start().context("on start").unwrap();
        while !self.is_window_open() {
            self.window_wrapper.glfw.poll_events();

            let _ = self.on_loop().context("on loop").unwrap();

            for (_, event) in glfw::flush_messages(&self.window_wrapper.events) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.close_window();
                    }

                    /*glfw::WindowEvent::Pos(..) => {
                        state.update_surface();
                        state.resize(state.context.size);
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        state.update_surface();
                        state.resize((width, height));
                    }*/
                    _ => {}
                }
            }
        }
    }

    fn on_loop(&mut self) -> Result<LoopState> {
        self.last_time = self.current_time;
        self.current_time = Instant::now();

        let mut delta_time = self.current_time - self.last_time;
        if delta_time > Duration::from_millis(100) {
            delta_time = Duration::from_millis(100);
        }

        while self.accumulator > self.update_timestep {
            let next = (self.update)(&mut self.game)?;
            if let LoopState::Exit(..) = next {
                return Ok(next);
            }
            self.accumulator -= self.update_timestep;
        }

        let next = (self.render)(&mut self.game, delta_time)?;
        if let LoopState::Exit(..) = next {
            return Ok(next);
        }
        self.accumulator += delta_time;
        Ok(LoopState::Continue)
    }

    pub fn close_window(&mut self) {
        self.window_wrapper.window.set_should_close(true);
    }

    pub fn is_window_open(&mut self) -> bool {
        self.window_wrapper.window.should_close()
    }

    fn on_start(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_end(&mut self, code: i32) -> Result<()> {
        std::process::exit(code)
    }
}
