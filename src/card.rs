use crate::assets::ImgID;
use crate::buffs::BuffType;
use crate::gui::CursorMode;
use crate::map::GameMap;
use crate::pile_overlay::PileOverlay;
use crate::playing_state::PlayingState;
use crate::shop_overlay::ShopOverlay;
use crate::tower::Tower;
use crate::wave::WaveStatus;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    Tower,
    SellTower,
    DamageEnemy,
    Shop,
    Coin(usize),
    Take2,
    Buff(BuffType),
    NextWave,
    DrawPile,
    DiscardPile,
}

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::Tower => ImgID::Archer,
            CardType::SellTower => ImgID::SellTower,
            CardType::DamageEnemy => ImgID::DamageEnemy,
            CardType::Shop => ImgID::Shop,
            CardType::Coin(a) => ImgID::Coin(*a),
            CardType::Take2 => ImgID::Take2,
            CardType::Buff(BuffType::Freeze) => ImgID::Freeze,
            CardType::Buff(BuffType::RPM) => ImgID::RPM,
            CardType::Buff(BuffType::Range) => ImgID::Range,
            CardType::Buff(BuffType::Damage) => ImgID::Damage,
            CardType::Buff(BuffType::Aura) => ImgID::Aura,
            CardType::NextWave => ImgID::NextWave,
            CardType::DrawPile => ImgID::DrawPile,
            CardType::DiscardPile => ImgID::DiscardPile,
        }
    }

    pub fn get_preview_image_id(&self) -> ImgID {
        return self.get_image_id();
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            CardType::Empty => "",
            CardType::Tower => "Builds a tower",
            CardType::SellTower => "Destroys a tower",
            CardType::DamageEnemy => "Damages all enemies in a given range",
            CardType::Shop => "Buy new cards",
            CardType::Coin(1) => "Produces 10 Gold",
            CardType::Coin(2) => "Produces 100 Gold",
            CardType::Coin(3) => "Produces 1000 Gold",
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => "Draw 2 more cards",
            CardType::Buff(BuffType::Freeze) => "Slows down enemies",
            CardType::Buff(BuffType::Range) => "Increases range",
            CardType::Buff(BuffType::Damage) => "Increases damage",
            CardType::Buff(BuffType::RPM) => "Increases rpm",
            CardType::Buff(BuffType::Aura) => "Increases stats of nearby towers",
            CardType::NextWave => "Immediatly starts next wave",
            CardType::DrawPile => "Look at your draw pile",
            CardType::DiscardPile => "Look at you discard pile",
        }
    }

    pub fn activation_cost(&self, state: &PlayingState) -> usize {
        match self {
            CardType::Empty => 0,
            CardType::Tower => state.towers.stats.price,
            CardType::SellTower => 0,
            CardType::DamageEnemy => 150,
            CardType::Shop => 0,
            CardType::Coin(1) => 0,
            CardType::Coin(2) => 0,
            CardType::Coin(3) => 0,
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => 10,
            CardType::Buff(BuffType::Freeze) => 10,
            CardType::Buff(BuffType::Damage) => 10,
            CardType::Buff(BuffType::RPM) => 10,
            CardType::Buff(BuffType::Range) => 10,
            CardType::Buff(BuffType::Aura) => 10,
            CardType::NextWave => 0,
            CardType::DrawPile => 0,
            CardType::DiscardPile => 0,
        }
    }

    pub fn aquisition_cost(&self, _state: &PlayingState) -> usize {
        match self {
            CardType::Empty => 0,
            CardType::Tower => 60,
            CardType::SellTower => 50,
            CardType::DamageEnemy => 100,
            CardType::Shop => 100,
            CardType::Coin(1) => 50,
            CardType::Coin(2) => 500,
            CardType::Coin(3) => 5000,
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => 500,
            CardType::NextWave => 50,
            CardType::Buff(BuffType::Freeze) => 100,
            CardType::Buff(BuffType::Damage) => 100,
            CardType::Buff(BuffType::RPM) => 100,
            CardType::Buff(BuffType::Range) => 100,
            CardType::Buff(BuffType::Aura) => 300,
            CardType::DrawPile => 0,
            CardType::DiscardPile => 0,
        }
    }

    pub fn select(&self, state: &mut PlayingState, slot: usize) {
        match self {
            CardType::Empty => {}
            CardType::Tower => state.gui.set_cursor_card_effect(slot, self),
            CardType::SellTower => state.gui.set_cursor_card_effect(slot, self),
            CardType::DamageEnemy => state.gui.set_cursor_card_effect(slot, self),
            CardType::Shop => state.overlay_state = Some(Box::new(ShopOverlay::new())),
            CardType::DiscardPile => {
                state.overlay_state = Some(Box::new(PileOverlay::new(CardType::DiscardPile)))
            }
            CardType::DrawPile => {
                state.overlay_state = Some(Box::new(PileOverlay::new(CardType::DrawPile)))
            }
            CardType::Coin(a) => {
                state.player_mut().gold += (10 as usize).pow(*a as u32);
                state.player_mut().deck.card_used(slot);
                let cards = state.player().deck.hand.len();
                if slot > 0 && slot == cards {
                    state.gui.set_cursor(CursorMode::Actions(slot - 1));
                }
            }
            CardType::Take2 => {
                state.player_mut().deck.draw(2);
                state.player_mut().deck.card_used(slot);
            }
            CardType::Buff(BuffType::Freeze) => state.gui.set_cursor_card_effect(slot, self),
            CardType::Buff(BuffType::Damage) => state.gui.set_cursor_card_effect(slot, self),
            CardType::Buff(BuffType::Range) => state.gui.set_cursor_card_effect(slot, self),
            CardType::Buff(BuffType::RPM) => state.gui.set_cursor_card_effect(slot, self),
            CardType::Buff(BuffType::Aura) => state.gui.set_cursor_card_effect(slot, self),
            CardType::NextWave => {
                if let WaveStatus::Waiting(_) = state.waves.status {
                    state.waves.status = WaveStatus::Waiting(0);
                }
            }
        }
    }

    pub fn is_applicable(&self, state: &PlayingState, x: usize, y: usize) -> bool {
        if state.player().gold < self.activation_cost(state) {
            return false;
        }
        match self {
            CardType::Empty => return false,
            CardType::Tower => {
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
            CardType::Coin(_) => return false,
            CardType::Take2 => return false,
            CardType::NextWave => return false,
            CardType::Buff(b) => {
                return state.towers.has_building(x, y)
                    && state.towers.get_tower(x, y).unwrap().can_have_buff(b);
            }
            CardType::DrawPile => return false,
            CardType::DiscardPile => return false,
        }
    }

    pub fn activate(&self, state: &mut PlayingState, x: usize, y: usize) {
        state.player_mut().gold -= self.activation_cost(state);
        match self {
            CardType::Empty => {}
            CardType::Tower => {
                state.towers.spawn(Tower::new((x, y)));
                state.gui.set_cursor(CursorMode::Actions(0));
                let pos = GameMap::tile_center(x, y);
                state.effects.smoke(pos.x, pos.y)
            }
            CardType::SellTower => {
                state.towers.remove_tower(x, y);
                state.gui.set_cursor(CursorMode::Actions(0));
            }
            CardType::DamageEnemy => {
                for e in state.enemies.in_range(GameMap::tile_center(x, y), 80.0) {
                    state.enemies.damage(e, 150);
                }
                state.gui.set_cursor(CursorMode::Actions(0));
            }
            CardType::Shop => {}
            CardType::DrawPile => {}
            CardType::DiscardPile => {}
            CardType::Coin(_) => {}
            CardType::Take2 => {}
            CardType::NextWave => {}
            CardType::Buff(b) => {
                state.towers.add_buff_at_pos(x, y, *b);
                state.gui.set_cursor(CursorMode::Actions(0));
                let pos = GameMap::tile_center(x, y);
                state.effects.buff(pos.x, pos.y, b)
            }
        }
    }
}
