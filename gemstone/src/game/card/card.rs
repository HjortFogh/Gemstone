use std::fmt::Debug;

use super::{card_like::CardLike, GemArchtype};

/// A card represented using a single byte as such: `XX'Y'Z'WWWW`, where `XX`
/// indicates the value (mapped from range `[0..4)` to `[1..5)`), `Y` indicates
/// whether this card is a coin card, `Z` indicates whether this card has been
/// leveraged, and `WWWW` is the index of the card archtype, if it is a gem
/// card. Any `card` is considered null if the value is `3` and the card is a
/// coin card.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    pub const NULL: Card = Card(0xf0);
}

impl Default for Card {
    fn default() -> Self {
        Self::NULL
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            f.write_str("Card { null }")
        } else {
            f.debug_struct("Card")
                .field("value", &self.value())
                .field("is_coin", &self.is_coin())
                .field("is_leveraged", &self.is_leveraged())
                .field("archtype", &self.archtype().index())
                .finish()
        }
    }
}

impl CardLike for Card {
    #[inline]
    fn value(self) -> i8 {
        (self.0 >> 6) as i8 + 1
    }

    fn get_value(self) -> Option<i8> {
        if self.is_leveraged() {
            None
        } else {
            Some(self.value())
        }
    }

    fn with_value(mut self, value: i8) -> Self {
        self.0 = (self.0 & 0x3f) | (value - 1 << 6) as u8;
        self
    }

    #[inline]
    fn is_leveraged(self) -> bool {
        // the bit is 1 if this card is leveraged
        self.0 & 0x10 != 0
    }

    fn with_leverage(mut self, leverage: bool) -> Self {
        self.0 = self.0 & 0xef | (u8::from(leverage) << 4);
        self
    }

    #[inline]
    fn is_coin(self) -> bool {
        // the bit is 1 if this card is a coin card
        self.0 & 0x20 != 0
    }

    #[inline]
    fn archtype(self) -> GemArchtype {
        GemArchtype::from_index(self.0 & 0x0f)
    }

    fn with_type(mut self, card_type: Option<GemArchtype>) -> Self {
        match card_type {
            Some(archtype) => self.0 = (self.0 & 0xd0) | archtype.index(),
            None => self.0 = (self.0 & 0xf0) | 0x20,
        }
        self
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 & 0xf0 == 0xf0
    }
}

impl Card {
    /// TODO: docs
    pub fn new(value: i8, is_leveraged: bool, archtype: Option<GemArchtype>) -> Self {
        Self::default()
            .with_value(value)
            .with_leverage(is_leveraged)
            .with_type(archtype)
    }

    /// TODO: docs
    pub fn coin(value: i8) -> Self {
        Self::default().with_value(value).with_leverage(false)
    }

    pub fn gem(archtype: GemArchtype) -> Self {
        let gem = Self::default()
            .with_type(Some(archtype))
            .with_value(archtype.value());
        gem
    }

    pub fn gem_deck() -> [Card; 18] {
        let mut cards = [Card::default(); 18];
        for i in -2_i32..16 {
            let archtype = GemArchtype::from_index(i.max(0) as u8);
            cards[(i + 2) as usize] = Self::gem(archtype);
        }
        cards
    }
}