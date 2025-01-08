use super::Card;

#[derive(Clone)]
pub struct CardCollection<const N: usize> {
    cards: [Card; N],
    len: usize,
}

impl<const N: usize> CardCollection<N> {
    pub fn push(&mut self, card: Card, idx: usize) {
        assert!(self.len < N);
        self.cards[idx..].rotate_right(1);
        self.cards[idx] = card;
        self.len += 1;
    }

    pub fn push_back(&mut self, card: Card) {
        assert!(self.len < N);
        self.cards[self.len] = card;
        self.len += 1;
    }

    pub fn pop(&mut self, idx: usize) -> Card {
        let card = self.cards[idx];
        self.cards[idx] = Card::NULL;
        self.cards[idx..].rotate_left(1);
        card
    }
}
