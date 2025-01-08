use crate::{
    game::{Card, CardChoice},
    CardIterator,
};

/// TODO: write docs for entire file
#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerInventory {
    cards: [Card; 21],
    size: u8,
}

impl PlayerInventory {
    pub fn with_coins() -> Self {
        let mut default = Self::default();
        default.add(Card::coin(1));
        default.add(Card::coin(2));
        default.add(Card::coin(3));
        default
    }

    pub fn as_slice(&self) -> &[Card] {
        &self.cards[0..self.size as usize]
    }

    pub fn add(&mut self, card: Card) {
        self.cards[self.size as usize] = card;
        self.size += 1;
    }

    pub fn iter(&self) -> impl CardIterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn iter_mut(&mut self) -> impl CardIterator<Item = &mut Card> {
        self.cards.iter_mut()
    }

    pub fn choose(&self, choice: CardChoice) -> impl CardIterator<Item = &Card> {
        self.cards.iter().choose_cards(choice)
    }

    pub fn choose_mut(&mut self, choice: CardChoice) -> impl CardIterator<Item = &mut Card> {
        self.cards.iter_mut().choose_cards(choice)
    }
}
