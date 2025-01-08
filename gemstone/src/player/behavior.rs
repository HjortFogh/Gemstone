use crate::{
    game::{CardChoice, GameInfo},
    BidValue,
};

pub trait PlayerBehavior {
    /// TODO: write documentation
    fn bid(&mut self, info: &GameInfo) -> BidValue;
    /// TODO: write documentation
    fn pick_card(&mut self, info: &GameInfo) -> (usize, CardChoice);
    /// TODO: write documentation
    fn reinvest(&mut self, info: &GameInfo) -> CardChoice;
}
