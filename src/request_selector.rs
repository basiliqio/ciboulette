use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug, Clone)]
pub enum CibouletteRequestSelector<'a> {
    Create(CibouletteCreateRequest<'a>),
    Read(CibouletteReadRequest<'a>),
    Update(CibouletteUpdateRequest<'a>),
    Delete(CibouletteDeleteRequest<'a>),
}

impl<'a> CibouletteRequestSelector<'a> {
    pub fn main_type(&self) -> &'a CibouletteResourceType<'a> {
        match self {
            CibouletteRequestSelector::Create(x) => x.path().main_type(),
            CibouletteRequestSelector::Read(x) => x.path().main_type(),
            CibouletteRequestSelector::Update(x) => x.resource_type(),
            CibouletteRequestSelector::Delete(x) => x.resource_type(),
        }
    }
}
