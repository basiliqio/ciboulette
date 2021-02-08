use super::*;

#[derive(Deserialize, Serialize, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    id: Cow<'a, str>,
    #[serde(default)]
    meta: Value,
}

impl<'a> CibouletteResourceIdentifier<'a> {
    pub fn new(id: Cow<'a, str>, type_: Cow<'a, str>, meta: Value) -> Self {
        CibouletteResourceIdentifier { id, type_, meta }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
}
