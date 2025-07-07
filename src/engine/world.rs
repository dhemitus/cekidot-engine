use crate::engine::{
    render_loop::{LoopState, RenderLoop},
    window::WindowWrapper,
};
use anyhow::Context;

pub struct World<'a> {
    render_loop: &'a mut RenderLoop<'a>,
    window_wrapper: WindowWrapper<'a>,
}

impl<'a> World<'a> {
    pub fn new(render_loop: &'a mut RenderLoop<'a>, window_wrapper: WindowWrapper<'a>) -> Self {
        Self {
            render_loop,
            window_wrapper,
        }
    }

    pub fn run(&mut self) {
        self.render_loop.on_start().context("on start").unwrap();
        let mut code = 0i32;
        while !self.window_wrapper.is_open() {
            self.window_wrapper.set_poll_events();

            let next = self.render_loop.on_loop().context("on loop").unwrap();

            if let LoopState::Exit(c) = next {
                code = c;
                self.window_wrapper.close();
                return;
            }

            for (_, event) in glfw::flush_messages(&self.window_wrapper.events()) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.render_loop.on_end(code).context("on end").unwrap();
                        code = 0;

                        self.window_wrapper.close();
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
}
