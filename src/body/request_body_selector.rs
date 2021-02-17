use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug, Clone)]
pub enum CibouletteRequestSelector<'a> {
    Create(CibouletteCreateRequestBody<'a>),
    Read(CibouletteReadRequestBody<'a>),
    Update(CibouletteUpdateRequestBody<'a>),
    Delete(CibouletteDeleteRequestBody),
}
