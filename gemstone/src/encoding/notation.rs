/*
-/!AESED/cf123;123;123
--- first player starts
--- first player bids 3
--- first player becomes the highest bidder
3/!AESED/fh123;c123;123
--- second player passes
3/!AESED/fh123;123;c123
--- third player passes
--- first player pays with coin 3
--- first player buys AE
--- second player becomes starting bidder
--- highest bid is reset
-/!SED/12!3AE;cf123;123
--- second player starts
--- second player bids 3
--- second player becomes the highest bidder
3/!SED/12!3AE;fh123;c123
--- third player bids 4
--- third player becomes the highest bidder
4/!SED/c12!3AE;f123;h123
--- first player is forced to pass
--- third player pays with coin 1, coin 3
--- third player buys SE
--- first player becomes starting bidder
--- highest bid is reset
-/!D/cf12!3AE;123;2!13SE
--- first player starts
--- first player bids 1
--- first player becomes the highest bidder
1/!D/fh12!3AE;c123;2!13SE
--- second player bids 2
--- second player becomes the highest bidder
2/!D/f12!3AE;h123;c2!13SE
--- third player is forced to pass
--- second player pays with coin 2
--- second player becomes starting reinvester
--- highest bid is reset
-//12!3AE;cf13!D2;2!13SE
--- second player selects coin 1, !D
-//12!3AE;f3D!12;c2!13SE
--- third player selects coin 2, !SE
-//c12!3AE;f3D!12;SE!123
--- first player selects coin 2, !AE
-//c1AE!23;f3D!12;SE!123
--- all coin cards becomes non-leveraged
--- new cards are drawn
--- third player becomes the starting bidder
-/EEASRR/123AE;123D;cf123SE

-/!AESED/cf123;123;123
3/!AESED/fh123;c123;123
3/!AESED/fh123;123;c123
-/!SED/12!3AE;cf123;123
3/!SED/12!3AE;fh123;c123
4/!SED/c12!3AE;f123;h123
-/!D/cf12!3AE;123;2!13SE
1/!D/fh12!3AE;c123;2!13SE
2/!D/f12!3AE;h123;c2!13SE
-//12!3AE;cf13!D2;2!13SE
-//12!3AE;f3D!12;c2!13SE
-//c12!3AE;f3D!12;SE!123
-//c1AE!23;f3D!12;SE!123
-/EEASRR/123AE;123D;cf123SE
*/

use std::fmt::Display;

use crate::{
    game::{Card, GameInfo},
    CardIterator, PlayerInventory,
};

/// TODO: docs
pub struct GemNotation(String);

impl Display for GemNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GemNotation {
    /// TODO: docs
    pub fn from_info(info: &GameInfo) -> Self {
        Self(Self::format(info))
    }

    pub fn to_info(&self) -> GameInfo {
        todo!()
    }

    /// TODO: docs
    pub fn inner(self) -> String {
        self.0
    }
}

const ARCHTYPE_CODES: [&str; 16] = [
    "D", "AA", "AE", "AS", "EE", "ER", "ET", "RA", "RR", "RT", "SE", "SR", "SS", "TA", "TS", "TT",
];

impl GemNotation {
    fn format(info: &GameInfo) -> String {
        format!(
            "{}/{}/{}",
            Self::format_highest_bid(info.highest_bid()),
            Self::format_cards(info.stack()),
            Self::format_inventories(info.inventories()),
        )
    }

    fn format_highest_bid(bid: i8) -> String {
        if bid < 0 {
            '-'.to_string()
        } else {
            bid.to_string()
        }
    }

    fn format_inventories(inventories: &[PlayerInventory]) -> String {
        inventories
            .iter()
            .map(|inv| Self::format_cards(inv.as_slice()))
            .collect::<Vec<String>>()
            .join(";")
    }

    fn format_cards(cards: &[Card]) -> String {
        let lhs = cards
            .iter()
            .cloned()
            .non_leveraged()
            .map(Self::format_card)
            .collect::<String>();
        let rhs = cards
            .iter()
            .cloned()
            .leveraged()
            .map(Self::format_card)
            .collect::<String>();
        match rhs.is_empty() {
            true => lhs,
            false => format!("{lhs}!{rhs}"),
        }
    }

    fn format_card(card: Card) -> String {
        if card.is_coin() {
            card.value().to_string()
        } else {
            ARCHTYPE_CODES[card.archtype().index() as usize].to_string()
        }
    }
}
