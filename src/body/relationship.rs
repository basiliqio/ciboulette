use super::*;

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    links: Option<CibouletteLink<'a>>,
    data: Option<CibouletteResourceIdentifierSelector<'a>>,
    meta: HashMap<Cow<'a, str>, Value>,
}
