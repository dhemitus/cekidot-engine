use crate::engine::{
    input::{InputState, KeyboardKey},
    render_loop::{LoopState, RenderLoop},
    window::WindowWrapper,
};
use anyhow::Context;

pub struct World<'a, Game, InputGame: InputState> {
    render_loop: &'a mut RenderLoop<'a, Game, InputGame>,
    window_wrapper: WindowWrapper<'a>,
}

impl<'a, Game, InputGame> World<'a, Game, InputGame>
where
    InputGame: InputState,
{
    pub fn new(
        render_loop: &'a mut RenderLoop<'a, Game, InputGame>,
        window_wrapper: WindowWrapper<'a>,
    ) -> Self {
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

            //            self.render_loop.input.handle_event();
            let next = self.render_loop.on_loop().context("on loop").unwrap();

            if let LoopState::Exit(c) = next {
                code = c;
                self.window_wrapper.close();
                return;
            }

            for (_, event) in glfw::flush_messages(&self.window_wrapper.events()) {
                //                self.render_loop.input.set_event(&event);
                self.render_loop.input.get_event(&event);

                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.render_loop.on_end(code).context("on end").unwrap();
                        code = 0;

                        self.window_wrapper.close();
                    }

                    glfw::WindowEvent::Pos(..) => {
                        self.render_loop.on_resize();
                        //state.update_surface();
                        //state.resize(state.context.size);
                    }
                    /*glfw::WindowEvent::FramebufferSize(width, height) => {

                        state.update_surface();
                        state.resize((width, height));
                    }*/
                    _ => {}
                }
            }
        }
    }
}
