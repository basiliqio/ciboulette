use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug, Clone)]
pub enum CibouletteRequestSelector<'a> {
    Create(CibouletteCreateRequest<'a>),
    Read(CibouletteReadRequest<'a>),
    Update(CibouletteUpdateRequest<'a>),
    Delete(CibouletteDeleteRequest<'a>),
}
