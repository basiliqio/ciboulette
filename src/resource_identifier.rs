use super::*;

#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    type_: Cow<'a, str>,
    id: Cow<'a, str>,
    meta: HashMap<Cow<'a, str>, Value>,
}

pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
    Null,
}
