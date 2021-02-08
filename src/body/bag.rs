use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug)]
pub struct CibouletteBag {
    map: BTreeMap<String, CibouletteResourceType>,
}

impl<'a> CibouletteBag {
    /// Create a new bag
    pub fn new(map: BTreeMap<String, CibouletteResourceType>) -> Self {
        CibouletteBag { map }
    }

    /// Get the inner map
    pub fn map(&self) -> &BTreeMap<String, CibouletteResourceType> {
        &self.map
    }
}
