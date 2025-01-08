use crate::game::{CardChoice, GameInfo};

pub trait PlayerBehavior {
    /// TODO: write documentation
    fn bid(&mut self, info: &GameInfo) -> i8;
    /// TODO: write documentation
    fn pick_card(&mut self, info: &GameInfo) -> (usize, CardChoice);
    /// TODO: write documentation
    fn reinvest(&mut self, info: &GameInfo) -> CardChoice;
}
