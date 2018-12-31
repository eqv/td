use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};

use crate::assets::{ImgID, Imgs};
use crate::card::CardType;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::towers::{Tower, TowerType};
use crate::utils::add_mod;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CursorMode {
    Build {
        x: usize,
        y: usize,
        t: TowerType,
    },
    Select {
        slot: usize,
    },
}

use self::CursorMode::*;

impl CursorMode {

    pub fn chancel(&self, _state: &GameState) -> Self {
        return self.clone();
    }

    pub fn up(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut y, .. } => *y = add_mod(*y,-1, state.map.ysize),
            Select { slot: ref mut x } => *x = add_mod(*x, -1, state.deck.hand.len()),
        }
        return res;
    }
    pub fn down(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut y, .. } => *y = add_mod(*y, 1, state.map.ysize),
            Select { slot: ref mut x } => *x = add_mod(*x, 1, state.deck.hand.len()),
        }
        return res;
    }
    pub fn left(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut x, .. } => *x = add_mod(*x, -1, state.map.xsize),
            Select { .. } => {}
        }
        return res;
    }
    pub fn right(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut x, .. } => *x = add_mod(*x, 1, state.map.xsize),
            Select { .. } => {}
        }
        return res;
    }
}

pub struct Gui {
    cursor_state: CursorMode,
}

impl Gui {
    pub fn new() -> Self {
        let cursor_state = CursorMode::Select { slot: 0 };
        return Self {
            cursor_state,
        };
    }

    pub fn set_cursor(&mut self, c: CursorMode) {
        self.cursor_state = c;
    }
    fn draw_map_cursor(
        &self,
        x: usize,
        y: usize,
        imgs: &Imgs,
        ctx: &mut Context,
    ) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            imgs.get(&ImgID::Cursor),
            graphics::DrawParam {
                // src: src,
                dest: GameMap::tile_pos(x, y),
                //rotation: self.zoomlevel,
                offset: Point2::new(1.0 / 22.0, 1.0 / 22.0),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    fn draw_build_preview(
        state: &GameState,
        x: usize,
        y: usize,
        t: TowerType,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let color = if state.map.is_buildable(x, y) && state.towers.is_buildable(x, y) {
            graphics::Color::new(0.2, 1.0, 0.2, 0.7)
        } else {
            graphics::Color::new(1.0, 0.2, 0.2, 0.7)
        };
        graphics::draw_ex(
            ctx,
            state.imgs.get(&t.get_image_id()),
            graphics::DrawParam {
                // src: src,
                dest: GameMap::tile_center(x, y),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                color: Some(color),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    fn draw_cards(state: &GameState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in state.deck.hand.iter().enumerate() {
            graphics::draw_ex(
                ctx,
                state.imgs.get(&card.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(500.0, 40.0 + (i as f32) * 80.0),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    fn draw_cards_cursor(&self, slot: usize, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            imgs.get(&ImgID::Cursor),
            graphics::DrawParam {
                dest: Point2::new(500.0, 40.0 + (slot as f32) * 80.0),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                ..Default::default()
            },
        )?;
        Ok(())
    }
    pub fn draw(state: &GameState, ctx: &mut Context) -> GameResult<()> {
        match state.gui.cursor_state {
            CursorMode::Build { x, y, t } => {
                state.gui.draw_map_cursor(x, y, &state.imgs, ctx)?;
                Gui::draw_build_preview(state, x, y, t, ctx)?;
            }
            CursorMode::Select { slot } => {
                state.gui.draw_cards_cursor(slot, &state.imgs, ctx)?;
            }
        }
        Gui::draw_cards(state, ctx)?;
        Ok(())
    }

    pub fn key_down(state: &mut GameState, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if keycode == Keycode::Up {
            state.gui.cursor_state = state.gui.cursor_state.up(state);
        }
        if keycode == Keycode::Down {
            state.gui.cursor_state = state.gui.cursor_state.down(state);
        }
        if keycode == Keycode::Left {
            state.gui.cursor_state = state.gui.cursor_state.left(state);
        }
        if keycode == Keycode::Right {
            state.gui.cursor_state = state.gui.cursor_state.right(state);
        }

        if keycode == Keycode::Escape {
            state.gui.cursor_state = state.gui.cursor_state.chancel(state);
        }

        if keycode == Keycode::Space {
            match state.gui.cursor_state {
                CursorMode::Build {
                    x,
                    y,
                    t,
                } => Gui::event_build(state, x, y, t),
                CursorMode::Select { slot } => Gui::event_activate(state, slot),
            }
        }
    }

    fn event_build(state: &mut GameState, x: usize, y: usize, t: TowerType) {
        if state.map.is_buildable(x, y) && state.towers.is_buildable(x, y) {
            state.towers.spawn(Tower::new(t, (x, y), 100, 100.0, 30));
            state.gui.cursor_state = CursorMode::Select { slot: 0 };
        }
    }

    fn event_activate(state: &mut GameState, slot: usize) {
        let card = state.deck.hand[slot].clone();
        state.deck.hand[slot] = CardType::Empty;
        card.activate(state);
    }
}
