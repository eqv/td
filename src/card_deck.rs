use crate::buffs::BuffType;
use crate::card::CardType;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct CardDeck {
    pub hand: Vec<CardType>,
    pub actions: Vec<CardType>,
    pub deck: Vec<CardType>,
    pub discard: Vec<CardType>,
}

impl CardDeck {
    pub fn all() -> Self {
        let hand = vec![];
        let deck = vec![
            CardType::SellTower,
            CardType::DamageEnemy,
            CardType::Coin(1),
            CardType::Coin(2),
            CardType::Coin(3),
            CardType::Take2,
            CardType::Buff(BuffType::Freeze),
            CardType::Buff(BuffType::Range),
            CardType::Buff(BuffType::Damage),
            CardType::Buff(BuffType::RPM),
            CardType::Buff(BuffType::Aura),
        ];
        let actions = vec![
            CardType::NextWave,
            CardType::Tower,
            CardType::Shop,
            CardType::DrawPile,
            CardType::DiscardPile,
        ];
        let discard = vec![];
        Self {
            hand,
            deck,
            actions,
            discard,
        }
    }

    pub fn new() -> Self {
        let hand = vec![];
        let deck = vec![];
        let discard = vec![];
        let actions = vec![
            CardType::NextWave,
            CardType::Tower,
            CardType::Shop,
            CardType::DrawPile,
            CardType::DiscardPile,
        ];
        Self {
            hand,
            deck,
            actions,
            discard,
        }
    }

    pub fn discard_all(&mut self) {
        self.discard
            .extend(self.hand.drain(..).filter(|c| c != &CardType::Empty));
    }

    pub fn card_used(&mut self, slot: usize) {
        if slot < self.hand.len() {
            assert!(self.hand[slot] != CardType::Empty);
            self.discard.push(self.hand[slot]);
            self.hand.remove(slot);
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.as_mut_slice().shuffle(&mut thread_rng());
    }

    pub fn get_selected_card(&self, slot: usize) -> Option<&CardType> {
        if slot < self.hand.len() {
            return self.hand.get(slot);
        }
        return self.actions.get(slot - self.hand.len());
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
