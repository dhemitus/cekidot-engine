use crate::engine::input::{EventState, InputState, KeyboardAction, KeyboardKey, KeyboardState};
use crate::engine::render_loop::LoopState;
use glfw::Action;
use glfw::Key;
use glfw::WindowEvent;
use std::collections::HashSet;

pub fn key_mapping(key: Key) -> Option<KeyboardKey> {
    match key {
        Key::GraveAccent => Some(KeyboardKey::GRAVE),
        Key::Escape => Some(KeyboardKey::ESCAPE),
        Key::Space => Some(KeyboardKey::SPACE),
        Key::Enter => Some(KeyboardKey::ENTER),
        Key::Tab => Some(KeyboardKey::TAB),
        Key::Delete => Some(KeyboardKey::DELETE),
        Key::Backspace => Some(KeyboardKey::BACKSPACE),
        Key::Insert => Some(KeyboardKey::INSERT),
        Key::Right => Some(KeyboardKey::RIGHT),
        Key::Left => Some(KeyboardKey::LEFT),
        Key::Down => Some(KeyboardKey::DOWN),
        Key::Up => Some(KeyboardKey::UP),
        Key::PageUp => Some(KeyboardKey::PAGEUP),
        Key::PageDown => Some(KeyboardKey::PAGEDOWN),
        Key::Home => Some(KeyboardKey::HOME),
        Key::End => Some(KeyboardKey::END),
        Key::CapsLock => Some(KeyboardKey::CAPSLOCK),
        Key::ScrollLock => Some(KeyboardKey::SCROLLLOCK),
        Key::NumLock => Some(KeyboardKey::NUMLOCK),
        Key::PrintScreen => Some(KeyboardKey::PRINTSCREEN),
        Key::Pause => Some(KeyboardKey::PAUSE),

        Key::Num0 => Some(KeyboardKey::ZERO),
        Key::Num1 => Some(KeyboardKey::ONE),
        Key::Num2 => Some(KeyboardKey::TWO),
        Key::Num3 => Some(KeyboardKey::THREE),
        Key::Num4 => Some(KeyboardKey::FOUR),
        Key::Num5 => Some(KeyboardKey::FIVE),
        Key::Num6 => Some(KeyboardKey::SIX),
        Key::Num7 => Some(KeyboardKey::SEVEN),
        Key::Num8 => Some(KeyboardKey::EIGHT),
        Key::Num9 => Some(KeyboardKey::NINE),

        Key::A => Some(KeyboardKey::A),
        Key::B => Some(KeyboardKey::B),
        Key::C => Some(KeyboardKey::C),
        Key::D => Some(KeyboardKey::D),
        Key::E => Some(KeyboardKey::E),
        Key::F => Some(KeyboardKey::F),
        Key::G => Some(KeyboardKey::G),
        Key::H => Some(KeyboardKey::H),
        Key::I => Some(KeyboardKey::I),
        Key::J => Some(KeyboardKey::J),
        Key::K => Some(KeyboardKey::K),
        Key::L => Some(KeyboardKey::L),
        Key::M => Some(KeyboardKey::M),
        Key::N => Some(KeyboardKey::N),
        Key::O => Some(KeyboardKey::O),
        Key::P => Some(KeyboardKey::P),
        Key::Q => Some(KeyboardKey::Q),
        Key::R => Some(KeyboardKey::R),
        Key::S => Some(KeyboardKey::S),
        Key::T => Some(KeyboardKey::T),
        Key::U => Some(KeyboardKey::U),
        Key::V => Some(KeyboardKey::V),
        Key::W => Some(KeyboardKey::W),
        Key::X => Some(KeyboardKey::X),
        Key::Y => Some(KeyboardKey::Y),
        Key::Z => Some(KeyboardKey::Z),
        _ => None,
    }
}

pub fn action_mapping(action: Action) -> KeyboardAction {
    match action {
        Action::Press => KeyboardAction::PRESS,
        Action::Release => KeyboardAction::RELEASE,
        Action::Repeat => KeyboardAction::REPEAT,
        _ => KeyboardAction::ELSE,
    }
}

pub fn capture_event(event: &WindowEvent) -> Option<EventState> {
    let mut key_action: Option<EventState> = None;
    match event {
        WindowEvent::Key(k, _, a, _) => {
            key_action = Some(EventState {
                action: action_mapping(*a),
                key: key_mapping(*k),
            });
        }
        _ => {}
    }

    key_action
}

pub struct GlfwInputState {
    key_down: HashSet<KeyboardKey>,
    key_pressed_update: HashSet<KeyboardKey>,
    key_released_update: HashSet<KeyboardKey>,
    clear_key: bool,
    event: Option<EventState>,
}

impl GlfwInputState {
    pub fn new() -> Self {
        Self {
            key_down: HashSet::new(),
            key_pressed_update: HashSet::new(),
            key_released_update: HashSet::new(),
            clear_key: true,
            event: None,
        }
    }

    /*fn on_events(&mut self, event: &'a Option<EventState>) {
        self.event = event;
    }*/
}

impl InputState for GlfwInputState {
    fn handle_event(&mut self) {
        if self.clear_key {
            self.key_pressed_update.clear();
            self.key_released_update.clear();
            self.clear_key = false;
        }

        match &self.event {
            Some(action_key) => match action_key {
                EventState { action: a, key: k } => {
                    if *a == KeyboardAction::PRESS || *a == KeyboardAction::REPEAT {
                        if let Some(k) = k {
                            if !self.key_down.contains(&k) {
                                self.key_pressed_update.insert(*k);
                            }
                            self.key_down.insert(*k);
                        }
                    } else if *a == KeyboardAction::ELSE || *a == KeyboardAction::RELEASE {
                        if let Some(k) = k {
                            if self.key_down.contains(&k) {
                                self.key_released_update.insert(*k);
                            }
                            self.key_down.remove(k);
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
    }

    fn on_event(&mut self, event: &Option<EventState>) -> &Option<EventState> {
        self.event = *event;
        &self.event
    }

    fn on_start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_next(&mut self) -> anyhow::Result<LoopState> {
        self.clear_key = true;

        Ok(LoopState::Continue)
    }

    fn on_end(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl KeyboardState for GlfwInputState {
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.key_pressed_update.contains(&key)
    }

    fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.key_down.contains(&key)
    }

    fn is_key_released(&self, key: KeyboardKey) -> bool {
        self.key_released_update.contains(&key)
    }

    fn is_key_up(&self, key: KeyboardKey) -> bool {
        !self.key_down.contains(&key)
    }
}
