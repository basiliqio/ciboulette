use super::*;

#[derive(Deserialize, Serialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationship<'a> {
    links: Option<CibouletteLink<'a>>,
    data: Option<CibouletteResourceIdentifierSelector<'a>>,
    #[serde(default)]
    meta: HashMap<Cow<'a, str>, Value>,
}
