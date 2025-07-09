use crate::engine::input::{EventState, InputState, KeyboardAction, KeyboardKey, KeyboardState};
use crate::engine::render_loop::LoopState;
use glfw::Action;
use glfw::Key;
use glfw::WindowEvent;
use std::collections::HashSet;

pub fn key_mapping(key: Key) -> Option<KeyboardKey> {
    match key {
        Key::Escape => Some(KeyboardKey::Escape),
        Key::Space => Some(KeyboardKey::SPACE),
        Key::A => Some(KeyboardKey::A),
        Key::D => Some(KeyboardKey::D),
        Key::S => Some(KeyboardKey::S),
        Key::Q => Some(KeyboardKey::Q),
        Key::W => Some(KeyboardKey::W),
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

pub trait GlfwHandleState {}

pub struct GlfwInputState {
    key_down: HashSet<KeyboardKey>,
    key_pressed_update: HashSet<KeyboardKey>,
    key_released_update: HashSet<KeyboardKey>,
    clear_key: bool,
    key_action: Option<EventState>,
}

impl GlfwInputState {
    pub fn new() -> Self {
        Self {
            key_down: HashSet::new(),
            key_pressed_update: HashSet::new(),
            key_released_update: HashSet::new(),
            clear_key: true,
            key_action: None,
        }
    }
}

impl InputState for GlfwInputState {
    fn handle_event(&mut self) {
        if self.clear_key {
            self.key_pressed_update.clear();
            self.key_released_update.clear();
            self.clear_key = false;
        }

        match &self.key_action {
            Some(ak) => match ak {
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

    fn event(&mut self, event: Option<EventState>) {
        self.key_action = event;
    }

    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn next(&mut self) -> anyhow::Result<LoopState> {
        self.clear_key = true;

        Ok(LoopState::Continue)
    }

    fn end(&mut self) -> anyhow::Result<()> {
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
