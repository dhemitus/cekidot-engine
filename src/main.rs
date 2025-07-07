use cekidot::engine::state::Game;

fn main() {
    let mut game = Game::new();
    pollster::block_on(cekidot::run(120, 640, 480, "glfw loop", &mut game))
}
