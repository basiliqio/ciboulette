use super::*;
mod deserializing;
mod error;
pub use error::CibouletteSelectorError;

/// Selector between a single object T or a list of object T
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CibouletteSelector<T> {
    Single(T),
    Multi(Vec<T>),
}

impl<T> CibouletteSelector<T> {
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
