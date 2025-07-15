pub mod glfw_input;

use crate::engine::render_loop::LoopState;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,

    GRAVE,
    ESCAPE,
    SPACE,
    ENTER,
    TAB,
    DELETE,
    BACKSPACE,
    INSERT,
    RIGHT,
    LEFT,
    DOWN,
    UP,
    PAGEUP,
    PAGEDOWN,
    HOME,
    END,
    CAPSLOCK,
    SCROLLLOCK,
    NUMLOCK,
    PRINTSCREEN,
    PAUSE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardAction {
    PRESS,
    RELEASE,
    REPEAT,
    ELSE,
}

#[derive(Debug, Clone, Copy)]
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
    //fn event(&self) -> &Option<EventState>;
    fn on_event(&mut self, event: &Option<EventState>) -> &Option<EventState>;
    fn on_start(&mut self) -> Result<()>;
    fn on_next(&mut self) -> Result<LoopState>;
    fn on_end(&mut self) -> Result<()>;
}
