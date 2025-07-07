use glfw::{Context, Glfw, GlfwReceiver, Window, WindowEvent};

pub struct WindowWrapper<'a> {
    pub glfw: &'a mut Glfw,
    events: &'a GlfwReceiver<(f64, WindowEvent)>,
    window: &'a mut Window,
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

    pub fn set_poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn is_open(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn events(&mut self) -> &'a GlfwReceiver<(f64, WindowEvent)> {
        self.events
    }
}
