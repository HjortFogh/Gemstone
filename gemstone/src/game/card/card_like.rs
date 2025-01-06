use super::GemArchtype;

/// A generic trait for the `Card`-struct.
pub trait CardLike: Copy {
    /// Returns the value of this card as if it where non-leveraged.
    /// Notice that this function will still return the full value
    /// even if a card has been leveraged.
    fn value(self) -> i8;

    fn get_value(self) -> Option<i8>;

    fn with_value(self, value: i8) -> Self;

    /// Returns a boolean indicating whether this card has been leveraged.
    fn is_leveraged(self) -> bool;

    fn with_leverage(self, leverage: bool) -> Self;

    /// Returns a boolean indicating whether this card has been leveraged.
    fn is_coin(self) -> bool;

    /// Returns the card [`GemArchtype`]. Notice that this is only valid if
    /// this card is not a coin card.
    fn archtype(self) -> GemArchtype;

    fn with_type(self, card_type: Option<GemArchtype>) -> Self;

    /// Returns whether this card is considered null.
    fn is_null(self) -> bool;
}
