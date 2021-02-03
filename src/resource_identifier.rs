use super::*;

#[derive(Deserialize, Serialize, Debug, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    type_: Cow<'a, str>,
    id: Cow<'a, str>,
    meta: HashMap<Cow<'a, str>, Value>,
}

impl<'a> CibouletteResourceIdentifier<'a> {
    pub fn new(id: Cow<'a, str>, type_: Cow<'a, str>, meta: HashMap<Cow<'a, str>, Value>) -> Self {
        CibouletteResourceIdentifier { id, type_, meta }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
    Null,
}
