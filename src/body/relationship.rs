use super::*;

#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationship<'a> {
    links: Option<CibouletteLink<'a>>,
    data: Option<CibouletteResourceIdentifierSelector<'a>>,
    meta: HashMap<Cow<'a, str>, Value>,
}
