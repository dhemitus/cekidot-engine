pub mod engine;

use glfw::fail_on_errors;

//use crate::engine::state::Game;
use crate::engine::world::World;
use crate::engine::{
    render_loop::{RenderFn, RenderLoop, UpdateFn},
    window::WindowWrapper,
};

pub async fn run<'a, Game>(
    fps: usize,
    width: usize,
    height: usize,
    title: &str,
    game: &'a mut Game,
    update: UpdateFn<Game>,
    render: RenderFn<Game>,
) {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            title,
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    let window_wrapper: WindowWrapper = WindowWrapper::new(&mut glfw, &events, &mut window);

    let mut render_loop = RenderLoop::<Game>::new(fps, game, update, render);

    let mut world = World::new(&mut render_loop, window_wrapper);
    world.run();
}
