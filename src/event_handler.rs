use ggez::event::{self, KeyCode, KeyMods};
use ggez::{Context, GameResult};

use crate::assets::Data;

pub enum StateTransition {
    Stay,
    Next(Box<GameState>),
    Exit,
    Return,
}

pub trait GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn update(&mut self, ctx: &mut Context) -> GameResult<StateTransition>;
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) -> StateTransition;
    fn set_data(&mut self, data: Data);
    fn take_data(&mut self) -> Data;
}

pub struct GameEventHandler {
    state: Box<GameState>,
}

impl GameEventHandler {
    pub fn new(state: Box<GameState>) -> Self {
        return Self { state };
    }
    pub fn use_state(&mut self, mut new_state: Box<GameState>) {
        new_state.set_data(self.state.take_data());
        self.state = new_state;
    }
}

impl event::EventHandler for GameEventHandler {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state.update(ctx)? {
            StateTransition::Stay => {}
            StateTransition::Return => unreachable!(),
            StateTransition::Next(state) => self.use_state(state),
            StateTransition::Exit => {
                ggez::event::quit(ctx);
            }
        }

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        match self.state.key_down_event(ctx, keycode, keymod, repeat) {
            StateTransition::Stay => {}
            StateTransition::Return => unreachable!(),
            StateTransition::Next(state) => self.use_state(state),
            StateTransition::Exit => {
                ggez::event::quit(ctx);
            }
        }
    }
}
