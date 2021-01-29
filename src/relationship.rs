use super::*;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationship<'a> {
    links: Option<CibouletteLink<'a>>,
    data: Option<CibouletteResourceIdentifierSelector<'a>>,
    meta: HashMap<Cow<'a, str>, Value>,
}
