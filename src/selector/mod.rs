use super::*;
mod deserializing;
mod error;
mod iter;

#[cfg(test)]
mod tests;

pub use error::CibouletteSelectorError;
pub use iter::CibouletteSelectorIterator;

/// Selector between a single object T or a list of object T
#[derive(Debug, Clone, Serialize, Hash, PartialEq, Ord, Eq, PartialOrd)]
#[serde(untagged)]
pub enum CibouletteSelector<T> {
    Single(T),
    Multi(Vec<T>),
}

impl<T> CibouletteSelector<T> {
    /// Create a new [CibouletteSelector](CibouletteSelector) with a single value
    pub fn new_single(val: T) -> Self {
        CibouletteSelector::Single(val)
    }

    /// Create a new [CibouletteSelector](CibouletteSelector) with multiple values
    pub fn new_multi(val: Vec<T>) -> Self {
        CibouletteSelector::Multi(val)
    }
    /// Get the length of the value(s)
    pub fn len(&self) -> usize {
        match self {
            CibouletteSelector::Single(_) => 1,
            CibouletteSelector::Multi(list) => list.len(),
        }
    }

    /// Return true if the values are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a value by its index
    pub fn get(&self, idx: usize) -> Result<&T, CibouletteSelectorError> {
        match self {
            CibouletteSelector::Single(el) if idx == 0 => Ok(el),
            CibouletteSelector::Multi(el) if idx < el.len() => Ok(&el[idx]),
            _ => Err(CibouletteSelectorError::OutOfBound(idx)),
        }
    }

    /// Push another element in this value.
    ///
    /// It'll convert the enum to the `Multi` variant
    /// if it's originally a `Single` variant
    pub fn push(&mut self, val: T) {
        match self {
            CibouletteSelector::Single(_) => {
                let mut new_self = CibouletteSelector::Multi(Vec::with_capacity(2));

                std::mem::swap(self, &mut new_self);
                let first_value = match new_self {
                    CibouletteSelector::Single(el) => el,
                    _ => unreachable!(),
                };
                self.push(first_value);
                self.push(val);
            }
            CibouletteSelector::Multi(list) => list.push(val),
        }
    }
}

impl<T> From<T> for CibouletteSelector<T> {
    fn from(val: T) -> Self {
        CibouletteSelector::Single(val)
    }
}

impl<T> From<Vec<T>> for CibouletteSelector<T> {
    fn from(vals: Vec<T>) -> Self {
        CibouletteSelector::Multi(vals)
    }
}
