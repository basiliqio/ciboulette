use super::*;

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: Option<CibouletteResourceIdentifierSelector<'a>>,
    pub meta: HashMap<Cow<'a, str>, Value>,
}
