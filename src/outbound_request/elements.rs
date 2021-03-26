use super::*;
#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteResponseElements<'a, B: Serialize> {
    pub(crate) id: Cow<'a, str>,
    pub(crate) type_: Cow<'a, str>,
    pub(crate) data: Option<B>,
    pub(crate) related: Option<CibouletteResourceIdentifier<'a>>,
}
