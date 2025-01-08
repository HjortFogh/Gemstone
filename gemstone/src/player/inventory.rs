use std::ops::{Deref, DerefMut};

use crate::{Card, CardCollection};

#[derive(Clone)]
pub struct PlayerInventory(CardCollection<21>);

impl AsRef<[Card]> for PlayerInventory {
    fn as_ref(&self) -> &[Card] {
        self.0.as_ref()
    }
}

impl Deref for PlayerInventory {
    type Target = CardCollection<21>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PlayerInventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        let mut inv = PlayerInventory(CardCollection::default());
        inv.push_back(Card::coin(1));
        inv.push_back(Card::coin(2));
        inv.push_back(Card::coin(3));
        inv
    }
}
