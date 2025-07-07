use std::time::Duration;

#[derive(Debug)]
pub struct Game {
    pub update_called: usize,
    pub render_called: usize,
    pub time_passed: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self {
            update_called: 0,
            render_called: 0,
            time_passed: Duration::default(),
        }
    }
}
