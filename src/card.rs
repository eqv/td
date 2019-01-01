use crate::assets::ImgID;
use crate::game_state::GameState;
use crate::gui::CursorMode;
use crate::map::GameMap;
use crate::shop_overlay::ShopOverlay;
use crate::towers::{Tower, TowerType};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    Build(TowerType),
    SellTower,
    DamageEnemy,
    Shop,
}
use self::CardType::*;

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::Build(TowerType::Cannon) => ImgID::Cannon,
            CardType::Build(TowerType::Archer) => ImgID::Archer,
            CardType::SellTower => ImgID::SellTower,
            CardType::DamageEnemy => ImgID::DamageEnemy,
            CardType::Shop => ImgID::Shop,
        }
    }

    pub fn get_preview_image_id(&self) -> ImgID {
        return self.get_image_id();
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            CardType::Empty => "",
            CardType::Build(TowerType::Cannon) => "Builds a cannon tower",
            CardType::Build(TowerType::Archer) => "Builds an archer tower",
            CardType::SellTower => "Allows you to destroy a tower",
            CardType::DamageEnemy => "Damages all enemies in a given range",
            CardType::Shop => "Allows you to buy one card",
        }
    }

    pub fn select(&self, state: &mut GameState, slot: usize) {
        match self {
            CardType::Empty => {}
            CardType::Build(_) => state.gui.set_cursor_card_effect(slot, self),
            CardType::SellTower => state.gui.set_cursor_card_effect(slot, self),
            CardType::DamageEnemy => state.gui.set_cursor_card_effect(slot, self),
            CardType::Shop => state.overlay_state = Some(Box::new(ShopOverlay::new(slot))),
        }
    }

    pub fn is_applicable(&self, state: &GameState, x: usize, y: usize) -> bool {
        match self {
            CardType::Empty => return false,
            CardType::Build(_) => {
                return state.map.is_buildable(x, y) && !state.towers.has_building(x, y);
            }
            CardType::SellTower => return state.towers.has_building(x, y),
            CardType::DamageEnemy => {
                return state
                    .enemies
                    .in_range(GameMap::tile_center(x, y), 80.0)
                    .len()
                    > 0;
            }
            CardType::Shop => return false,
        }
    }

    pub fn activate(&self, state: &mut GameState, x: usize, y: usize) {
        match self {
            CardType::Empty => {}
            CardType::Build(t) => {
                state.towers.spawn(Tower::new(*t, (x, y)));
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::SellTower => {
                state.towers.remove_tower(x, y);
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::DamageEnemy => {
                for e in state.enemies.in_range(GameMap::tile_center(x, y), 80.0) {
                    state.enemies.damage(e, 150);
                }
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::Shop => {}
        }
    }
}

pub struct CardDeck {
    pub hand: Vec<CardType>,
    pub deck: Vec<CardType>,
    pub discard: Vec<CardType>,
}

impl CardDeck {
    pub fn new() -> Self {
        let hand = vec![];
        let deck = vec![
            Build(TowerType::Cannon),
            Build(TowerType::Archer),
            DamageEnemy,
            SellTower,
            Shop,
        ];
        let discard = vec![];
        Self {
            hand,
            deck,
            discard,
        }
    }

    pub fn card_used(&mut self, slot: usize) {
        self.discard.push(self.hand[slot]);
        self.hand[slot] = CardType::Empty;
    }

    pub fn shuffle(&mut self) {
        self.deck.as_mut_slice().shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(card) = self.draw_one() {
                self.hand.push(card);
            }
        }
    }

    pub fn draw_one(&mut self) -> Option<CardType> {
        if self.deck.is_empty() {
            self.deck.append(&mut self.discard);
            self.shuffle()
        }
        return self.deck.pop();
    }
}
