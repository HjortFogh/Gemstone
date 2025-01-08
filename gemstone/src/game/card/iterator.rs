use super::{Card, CardChoice};

pub trait CardIterator
where
    Self: Iterator + Sized,
    Self::Item: AsRef<Card>,
{
    fn choose_cards(self, choice: CardChoice) -> impl CardIterator<Item = Self::Item> {
        self.enumerate()
            .filter(move |(i, _)| choice.check(*i))
            .map(|(_, card)| card)
    }

    fn leveraged(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| card.as_ref().is_leveraged())
    }

    fn non_leveraged(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| !card.as_ref().is_leveraged())
    }

    fn non_null(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| !card.as_ref().is_null())
    }

    fn capital(self) -> i8 {
        self.flat_map(|card| card.as_ref().get_value()).sum()
    }
}

impl<I> CardIterator for I
where
    I: Iterator + Sized,
    I::Item: AsRef<Card>,
{
}
