use super::*;

#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Default, Clone)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObjectBuilder<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelectorBuilder<'a>>,
    pub meta: Option<Value>,
}

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Serialize, Getters, MutGetters, Default, Clone)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'a>>,
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}
