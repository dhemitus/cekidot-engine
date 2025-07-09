pub mod engine;

use crate::engine::canvas::RenderableCanvas;
use crate::engine::input::InputState;
use crate::engine::world::World;
use crate::engine::{
    render_loop::{RenderFn, RenderLoop, UpdateFn},
    window::WindowWrapper,
};
use glfw::fail_on_errors;

pub async fn run<'a, Game, InputGame: InputState, CanvasGame: RenderableCanvas>(
    fps: usize,
    width: usize,
    height: usize,
    title: &str,
    game: &'a mut Game,
    input: &'a mut InputGame,
    canvas: CanvasGame,
    update: UpdateFn<Game, InputGame, CanvasGame>,
    render: RenderFn<Game, InputGame, CanvasGame>,
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

    let mut render_loop =
        RenderLoop::<Game, InputGame, CanvasGame>::new(fps, game, input, canvas, update, render);

    let mut world = World::new(&mut render_loop, window_wrapper);
    world.run();
}
