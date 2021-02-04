use super::*;

#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationship<'a> {
    links: Option<CibouletteLink<'a>>,
    data: Option<CibouletteResourceIdentifierSelector<'a>>,
    #[serde(default)]
    meta: HashMap<Cow<'a, str>, Value>,
}
