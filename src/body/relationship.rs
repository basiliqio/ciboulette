use super::*;

#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Default, Clone)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObjectBuilder<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelectorBuilder<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Serialize, Getters, MutGetters, Default, Clone)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}
