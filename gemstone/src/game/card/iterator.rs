use super::{Card, CardChoice};

macro_rules! card_iterator_impl {
    ($name:ident for $($bounds:tt)+) => {
        pub trait $name
        where
            Self: Iterator + Sized,
            Self::Item: $($bounds)+,
        {
            fn choose_cards(self, choice: CardChoice) -> impl Iterator<Item = Self::Item> {
                self.enumerate()
                    .filter(move |(i, _)| choice.check(*i))
                    .map(|(_, card)| card)
            }

            fn non_null(self) -> impl Iterator<Item = Self::Item> {
                self.filter(|card| !card.as_ref().is_null())
            }
        }

        impl<I> $name for I
        where
            I: Iterator + Sized,
            I::Item: $($bounds)+,
        {
        }
    };
}

card_iterator_impl!(CardIteratorRef for AsRef<Card>);
card_iterator_impl!(CardIteratorMut for AsMut<Card> + AsRef<Card>);
