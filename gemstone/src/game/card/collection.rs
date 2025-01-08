use std::ops::Range;

use rand::{seq::SliceRandom, Rng};

use super::{Card, CardChoice, CardIterator};

#[derive(Clone, Debug)]
pub struct CardCollection<const N: usize> {
    cards: [Card; N],
    len: usize,
}

impl<const N: usize> AsRef<[Card]> for CardCollection<N> {
    fn as_ref(&self) -> &[Card] {
        &self.cards[..self.len]
    }
}

impl<const N: usize> Default for CardCollection<N> {
    fn default() -> Self {
        Self {
            cards: [Card::NULL; N],
            len: 0,
        }
    }
}

impl<const N: usize> CardCollection<N> {
    pub fn new(mut cards: [Card; N]) -> Self {
        cards.sort_by_key(|card| card.is_null());
        Self {
            cards,
            len: Self::find_last(&cards),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    fn find_last(cards: &[Card]) -> usize {
        let mut start = 0;
        let mut end = cards.len();

        while start < end {
            let mid = start + ((end - start) >> 1);
            if cards[mid].is_null() {
                if !cards[mid - 1].is_null() {
                    return mid;
                }
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        cards.len()
    }

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
        self.len -= 1;
        card
    }

    pub fn copy_from<const M: usize>(
        &mut self,
        src: &CardCollection<M>,
        src_range: Range<usize>,
        dest_range: Range<usize>,
    ) {
        // TODO: bounds assertions
        self.len = self.len.max(dest_range.end);
        self.cards[dest_range].copy_from_slice(&src.cards[src_range]);
    }

    pub fn iter(&self) -> impl CardIterator<Item = &Card> {
        self.cards[..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> impl CardIterator<Item = &mut Card> {
        self.cards[..self.len].iter_mut()
    }

    pub fn choose(&self, choice: CardChoice) -> impl CardIterator<Item = &Card> {
        self.cards[..self.len].iter().choose_cards(choice)
    }

    pub fn choose_mut(&mut self, choice: CardChoice) -> impl CardIterator<Item = &mut Card> {
        self.cards[..self.len].iter_mut().choose_cards(choice)
    }

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.cards[..self.len].shuffle(rng);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
