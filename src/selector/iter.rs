use super::*;

/// An iterator over a [CibouletteSelector](CibouletteSelector)
pub enum CibouletteSelectorIterator<T> {
    Single(std::iter::Once<T>),
    Multi(std::vec::IntoIter<T>),
}

impl<T> Iterator for CibouletteSelectorIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CibouletteSelectorIterator::Single(el) => el.next(),
            CibouletteSelectorIterator::Multi(el) => el.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            CibouletteSelectorIterator::Single(_) => (1, Some(1)),
            CibouletteSelectorIterator::Multi(el) => {
                let len = el.len();

                (len, Some(len))
            }
        }
    }
}

impl<T> IntoIterator for CibouletteSelector<T> {
    type Item = T;
    type IntoIter = CibouletteSelectorIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CibouletteSelector::Single(el) => {
                CibouletteSelectorIterator::Single(std::iter::once(el))
            }
            CibouletteSelector::Multi(el) => CibouletteSelectorIterator::Multi(el.into_iter()),
        }
    }
}
