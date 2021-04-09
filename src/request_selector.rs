use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug, Clone)]
pub enum CibouletteRequestSelector<'request> {
    Create(CibouletteCreateRequest<'request>),
    Read(CibouletteReadRequest<'request>),
    Update(CibouletteUpdateRequest<'request>),
    Delete(CibouletteDeleteRequest<'request>),
}

impl<'request> CibouletteRequestSelector<'request> {
    pub fn main_type(&self) -> &CibouletteResourceType {
        match self {
            CibouletteRequestSelector::Create(x) => x.path().main_type(),
            CibouletteRequestSelector::Read(x) => x.path().main_type(),
            CibouletteRequestSelector::Update(x) => x.resource_type(),
            CibouletteRequestSelector::Delete(x) => x.resource_type(),
        }
    }
}
