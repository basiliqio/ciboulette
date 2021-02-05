use super::*;

#[derive(Clone, Debug)]
pub struct CibouletteBag {
    map: BTreeMap<String, CibouletteResourceType>,
}

impl<'a> CibouletteBag {
    pub fn new(map: BTreeMap<String, CibouletteResourceType>) -> Self {
        CibouletteBag { map }
    }

    pub fn map(&self) -> &BTreeMap<String, CibouletteResourceType> {
        &self.map
    }
}
