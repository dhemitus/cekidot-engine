use anyhow::Result;
use std::time::{Duration, Instant};

use crate::engine::{canvas::RenderableCanvas, input::InputState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopState {
    Continue,
    Exit(i32),
}

pub type UpdateFn<Game, InputGame, CanvasGame> =
    fn(&mut Game, &mut InputGame, &mut CanvasGame) -> Result<LoopState>;

pub type RenderFn<Game, InputGame, CanvasGame> =
    fn(&mut Game, &mut InputGame, &mut CanvasGame, Duration) -> Result<LoopState>;

pub struct RenderLoop<'a, Game, InputGame: InputState, CanvasGame: RenderableCanvas> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    game: &'a mut Game,
    pub input: &'a mut InputGame,
    canvas: CanvasGame,
    update: UpdateFn<Game, InputGame, CanvasGame>,
    render: RenderFn<Game, InputGame, CanvasGame>,
}

impl<'a, Game, InputGame, CanvasGame> RenderLoop<'a, Game, InputGame, CanvasGame>
where
    InputGame: InputState,
    CanvasGame: RenderableCanvas,
{
    pub fn new(
        fps: usize,
        game: &'a mut Game,
        input: &'a mut InputGame,
        canvas: CanvasGame,
        update: UpdateFn<Game, InputGame, CanvasGame>,
        render: RenderFn<Game, InputGame, CanvasGame>,
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
            canvas,
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
            let next = (self.input).on_next()?;
            if let LoopState::Exit(..) = next {
                return Ok(next);
            }

            let next = (self.update)(&mut self.game, self.input, &mut self.canvas)?;
            if let LoopState::Exit(..) = next {
                return Ok(next);
            }
            self.accumulator -= self.update_timestep;
        }

        let next = (self.render)(&mut self.game, self.input, &mut self.canvas, delta_time)?;
        if let LoopState::Exit(..) = next {
            return Ok(next);
        }
        self.accumulator += delta_time;
        Ok(LoopState::Continue)
    }

    pub fn on_resize(&mut self) {
        self.canvas.on_resize();
        //    self.game.resize(100, 100);
        //todo!()
    }

    pub fn on_start(&mut self) -> Result<()> {
        self.input.on_start()?;
        self.canvas.on_start()?;
        Ok(())
    }

    pub fn on_end(&mut self, code: i32) -> Result<()> {
        self.input.on_end()?;
        self.canvas.on_end()?;
        std::process::exit(code)
    }
}
