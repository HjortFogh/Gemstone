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

    fn coin_cards(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| card.as_ref().is_coin())
    }

    fn gem_cards(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| !card.as_ref().is_coin())
    }

    fn non_null(self) -> impl CardIterator<Item = Self::Item> {
        self.filter(|card| !card.as_ref().is_null())
    }

    fn capital(self) -> i8 {
        self.flat_map(|card| card.as_ref().get_value()).sum()
    }

    fn scalar_value(self) -> i8 {
        self.map(|card| card.as_ref().scalar_value()).sum()
    }
}

impl<I> CardIterator for I
where
    I: Iterator + Sized,
    I::Item: AsRef<Card>,
{
}
