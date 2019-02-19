use crate::algebra::{Point, Vector};
use crate::assets::ImgID;
extern crate rand;
use crate::buffs::BuffType;
use crate::card::CardType;
use crate::event_handler::StateTransition;
use crate::overlay_state::OverlayState;
use crate::playing_state::PlayingState;
use crate::utils::{self, add_mod};
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};

pub struct PileOverlay {
    cur_selected: usize,
    cards: Vec<CardType>,
}

impl PileOverlay {
    pub fn new(cards: Vec<CardType>) -> Self {
        let mut shuffled = cards.clone();
        thread_rng().shuffle(&mut shuffled);
        return Self {
            cur_selected: 0,
            cards: shuffled,
        };
    }

    fn get_drawing_offset(&self) -> f32 {
        if self.cur_selected > 5 {
            return (self.cur_selected - 5) as f32 * 80.0;
        }
        return 0.0;
    }

    fn draw_cards(&self, state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in self.cards.iter().enumerate() {
            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&ImgID::Card),
                graphics::DrawParam::default()
                    .dest(Point::new(
                        100.0,
                        40.0 + (i as f32) * 80.0 - self.get_drawing_offset(),
                    ))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;
            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&card.get_image_id()),
                graphics::DrawParam::default()
                    .dest(Point::new(
                        100.0,
                        40.0 + (i as f32) * 80.0 - self.get_drawing_offset(),
                    ))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;
            let cost = card.aquisition_cost(state);
            if cost > 0 {
                let desc = utils::text(state.data.as_ref().unwrap(), &format!("{}", cost));

                graphics::draw(
                    ctx,
                    &desc,
                    graphics::DrawParam::default()
                        .dest(Point::new(
                            67.0,
                            45.0 + (i as f32) * 80.0 - self.get_drawing_offset(),
                        ))
                        .scale(Vector::new(0.3, 0.3)),
                )?;
            }
        }
        Ok(())
    }

    fn draw_cursor(&self, state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        if self.cards.get(self.cur_selected).is_none() {
            return Ok(());
        }
        graphics::draw(
            ctx,
            state.data.as_ref().unwrap().get_i(&ImgID::Cursor),
            graphics::DrawParam::default()
                .dest(Point::new(
                    100.0,
                    40.0 + (self.cur_selected as f32) * 80.0 - self.get_drawing_offset(),
                ))
                .offset(Point::new(0.5, 0.5))
                .scale(Vector::new(4.0, 4.0)),
        )?;
        return Ok(());
    }

    fn draw_selected(&self, state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        let card = self.cards.get(self.cur_selected).unwrap_or(return Ok(()));
        graphics::draw(
            ctx,
            state.data.as_ref().unwrap().get_i(&card.get_image_id()),
            graphics::DrawParam::default()
                .dest(Point::new(300.0, 40.0))
                .offset(Point::new(0.0, 0.0))
                .scale(Vector::new(8.0, 8.0)),
        )?;
        let desc = utils::text(state.data.as_ref().unwrap(), &card.get_description());
        graphics::draw(
            ctx,
            &desc,
            graphics::DrawParam::default()
                .dest(Point::new(300.0, 200.0))
                .offset(Point::new(0.0, 0.0))
                .scale(Vector::new(0.2, 0.2)),
        )?;
        return Ok(());
    }
}

impl OverlayState for PileOverlay {
    fn update(&mut self, _state: &mut PlayingState) -> GameResult<StateTransition> {
        return Ok(StateTransition::Stay);
    }

    fn draw(&self, state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.1, 0.2, 0.4, 1.0));
        //graphics::set_color(ctx, graphics::WHITE)?;
        self.draw_cards(state, ctx)?;
        self.draw_cursor(state, ctx)?;
        self.draw_selected(state, ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        state: &mut PlayingState,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) -> StateTransition {
        match keycode {
            KeyCode::Up => {
                if self.cards.len() > 0 {
                    self.cur_selected = add_mod(self.cur_selected, -1, self.cards.len())
                }
            }
            KeyCode::Down => {
                if self.cards.len() > 0 {
                    self.cur_selected = add_mod(self.cur_selected, 1, self.cards.len())
                }
            }
            KeyCode::Escape => {
                return StateTransition::Return;
            }
            _ => {}
        }
        return StateTransition::Stay;
    }
}
