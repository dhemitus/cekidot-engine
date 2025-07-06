use glfw::{Context, Glfw, GlfwReceiver, Window, WindowEvent};
use std::time::Duration;

#[derive(Debug)]
pub struct Game<'a> {
    pub update_called: usize,
    pub render_called: usize,
    pub time_passed: Duration,
    pub window: &'a mut Window,
    glfw: &'a mut Glfw,
    events: &'a GlfwReceiver<(f64, WindowEvent)>,
}

impl<'a> Game<'a> {
    pub async fn new(
        window: &'a mut Window,
        glfw: &'a mut Glfw,
        events: &'a GlfwReceiver<(f64, WindowEvent)>,
    ) -> Self {
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Hidden);
        window.make_current();

        Self {
            update_called: 0,
            render_called: 0,
            time_passed: Duration::default(),
            window,
            glfw,
            events,
        }
    }

    pub fn run_window(&mut self, render_loop: fn()) {
        while !self.is_window_open() {
            self.glfw.poll_events();

            let ren = render_loop;
            ren();
            //render_loop.on_loop();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        let _ = &self.close_window();
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

    fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn is_window_open(&self) -> bool {
        self.window.should_close()
    }
}
