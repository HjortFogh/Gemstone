use std::{error::Error, fmt::Display, result};

// TODO: make all error names better / revisit

#[derive(Debug)]
pub enum GemError {
    /// Raised when number of players exceed four
    ReachedPlayerLimit,
    /// Raised when the number of players are less than two
    TooFewPlayers,
    /// Raised when the [`Game::run`](crate::game::Game::run) or [`Game::step`](crate::game::Game::step)-functions are called
    GameAlreadyOver,
    /// Raised when a player bids more than the total sum of all their non-leveraged gem cards
    CannotAffordBid,
    /// Raised when a player tries to pay for a bid with too few gem cards
    TooFewGemCards,
    /// Raised when a player tries to pay with an already-leveraged gem card
    TriedToUseLeveragedCard,
    /// Raised when a player tries to flip a non-leveraged card
    TriedToFlipNonLeveragedCard,
    /// Raised when a player tries to flip a coin card
    TriedToFlipCoinCard,
    /// Raised when a player cannot afford to flip the provided cards
    CannotAffortToFlip,
}

impl Display for GemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GemError {}

pub type Result<T> = result::Result<T, GemError>;
