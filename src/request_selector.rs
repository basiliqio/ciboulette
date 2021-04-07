use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug, Clone)]
pub enum CibouletteRequestSelector<'request, 'store> {
    Create(CibouletteCreateRequest<'request, 'store>),
    Read(CibouletteReadRequest<'request, 'store>),
    Update(CibouletteUpdateRequest<'request, 'store>),
    Delete(CibouletteDeleteRequest<'request, 'store>),
}

impl<'request, 'store> CibouletteRequestSelector<'request, 'store> {
    pub fn main_type(&'store self) -> &'store CibouletteResourceType<'store> {
        match self {
            CibouletteRequestSelector::Create(x) => x.path().main_type(),
            CibouletteRequestSelector::Read(x) => x.path().main_type(),
            CibouletteRequestSelector::Update(x) => x.resource_type(),
            CibouletteRequestSelector::Delete(x) => x.resource_type(),
        }
    }
}
