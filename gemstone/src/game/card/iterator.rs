use super::{card_like::CardLike, CardChoice};

pub trait CardIterator: Iterator + Sized {
    fn non_null(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.filter(|card| !card.is_null())
    }

    fn null(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.filter(|card| card.is_null())
    }

    fn choose_cards(self, choice: CardChoice) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.enumerate()
            .filter(move |(idx, _)| choice.check(*idx))
            .map(|(_, item)| item)
    }

    fn leveraged(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.filter(|card| card.is_leveraged())
    }

    fn non_leveraged(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.filter(|card| !card.is_leveraged())
    }

    fn capital(self) -> i8
    where
        Self: Sized,
        Self::Item: CardLike,
    {
        self.filter_map(|card| card.get_value()).sum()
    }
}

impl<I> CardIterator for I
where
    I: Iterator,
    I::Item: CardLike,
{
}
