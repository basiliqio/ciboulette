use super::*;

/// An iterator over a [CibouletteSelector](CibouletteSelector)
pub enum CibouletteSelectorIterator<T> {
    Single(std::iter::Once<T>),
    Multi(std::vec::IntoIter<T>),
}

/// An iterator over a [CibouletteSelector](CibouletteSelector)
pub enum CibouletteSelectorIteratorRef<'a, T> {
    Single(std::iter::Once<&'a T>),
    Multi(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for CibouletteSelectorIteratorRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CibouletteSelectorIteratorRef::Single(el) => el.next(),
            CibouletteSelectorIteratorRef::Multi(el) => el.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            CibouletteSelectorIteratorRef::Single(_) => (1, Some(1)),
            CibouletteSelectorIteratorRef::Multi(el) => {
                let len = el.len();

                (len, Some(len))
            }
        }
    }
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

impl<T> CibouletteSelector<T> {
    pub fn iter(&self) -> CibouletteSelectorIteratorRef<'_, T> {
        match self {
            CibouletteSelector::Single(el) => {
                CibouletteSelectorIteratorRef::Single(std::iter::once(&el))
            }
            CibouletteSelector::Multi(el) => CibouletteSelectorIteratorRef::Multi(el.iter()),
        }
    }
}
