/// Represents a choice of cards given a specific collection of `Card`-structs.
/// Note that a `CardChoice` is meaningless without a corresponding collection.
#[derive(Clone, Copy, Debug)]
pub struct CardChoice(u32);

impl CardChoice {
    pub const NONE: CardChoice = CardChoice(0);
    pub const ALL: CardChoice = CardChoice(u32::MAX);
}

// TODO: make unit tests

impl CardChoice {
    pub fn new(indices: &[usize]) -> Self {
        let mut x = 0;
        for &i in indices {
            x |= 1 << i;
        }
        Self(x)
    }

    /// Checks if a given card index is part of this `CardChoice`.
    pub fn check(&self, idx: usize) -> bool {
        self.0 & (1 << idx) != 0
    }
}
