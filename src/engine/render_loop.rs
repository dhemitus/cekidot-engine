use anyhow::Result;
use std::time::{Duration, Instant};

use crate::engine::input::InputState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopState {
    Continue,
    Exit(i32),
}

pub type UpdateFn<Game, InputGame> = fn(&mut Game, &mut InputGame) -> Result<LoopState>;

pub type RenderFn<Game, InputGame> = fn(&mut Game, &mut InputGame, Duration) -> Result<LoopState>;

pub struct RenderLoop<'a, Game, InputGame: InputState> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    game: &'a mut Game,
    pub input: &'a mut InputGame,
    update: UpdateFn<Game, InputGame>,
    render: RenderFn<Game, InputGame>,
}

impl<'a, Game, InputGame> RenderLoop<'a, Game, InputGame>
where
    InputGame: InputState,
{
    pub fn new(
        fps: usize,
        game: &'a mut Game,
        input: &'a mut InputGame,
        update: UpdateFn<Game, InputGame>,
        render: RenderFn<Game, InputGame>,
    ) -> Self {
        if fps == 0 {
            panic!("must be > 0");
        }
        Self {
            accumulator: Duration::default(),
            current_time: Instant::now(),
            last_time: Instant::now(),
            update_timestep: Duration::from_nanos((1_000_000_000f64 / fps as f64).round() as u64),
            game,
            input,
            update,
            render,
        }
    }

    pub fn on_loop(&mut self) -> Result<LoopState> {
        self.last_time = self.current_time;
        self.current_time = Instant::now();

        let mut delta_time = self.current_time - self.last_time;
        if delta_time > Duration::from_millis(100) {
            delta_time = Duration::from_millis(100);
        }

        while self.accumulator > self.update_timestep {
            let next = (self.input).next()?;
            if let LoopState::Exit(..) = next {
                return Ok(next);
            }

            let next = (self.update)(&mut self.game, self.input)?;
            if let LoopState::Exit(..) = next {
                return Ok(next);
            }
            self.accumulator -= self.update_timestep;
        }

        let next = (self.render)(&mut self.game, self.input, delta_time)?;
        if let LoopState::Exit(..) = next {
            return Ok(next);
        }
        self.accumulator += delta_time;
        Ok(LoopState::Continue)
    }

    pub fn on_resize(&mut self) {
        //    self.game.resize(100, 100);
        //todo!()
    }

    pub fn on_start(&mut self) -> Result<()> {
        self.input.start()?;
        Ok(())
    }

    pub fn on_end(&mut self, code: i32) -> Result<()> {
        self.input.end()?;
        std::process::exit(code)
    }
}
