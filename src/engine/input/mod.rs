pub mod glfw_input;

use crate::engine::render_loop::LoopState;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A,
    D,
    S,
    W,
    Escape,
    SPACE,
    Q,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardAction {
    PRESS,
    RELEASE,
    REPEAT,
    ELSE,
}

pub struct EventState {
    pub key: Option<KeyboardKey>,
    pub action: KeyboardAction,
}

pub trait KeyboardState {
    fn is_key_pressed(&self, key: KeyboardKey) -> bool;
    fn is_key_down(&self, key: KeyboardKey) -> bool;
    fn is_key_released(&self, key: KeyboardKey) -> bool;
    fn is_key_up(&self, key: KeyboardKey) -> bool;
}

pub trait InputState: KeyboardState {
    fn handle_event(&mut self);
    fn event(&mut self, event: Option<EventState>);
    fn start(&mut self) -> Result<()>;
    fn next(&mut self) -> Result<LoopState>;
    fn end(&mut self) -> Result<()>;
}
