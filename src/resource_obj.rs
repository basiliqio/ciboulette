use super::*;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceObject<'a> {
    id: Option<Cow<'a, str>>,
    type_: Cow<'a, str>,
    attributes: Option<CibouletteResourceSchemaValue<'a>>,
    relationships: Option<CibouletteRelationship<'a>>,
    links: Option<CibouletteLink<'a>>,
    meta: HashMap<Cow<'a, str>, Value>,
}
