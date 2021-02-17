use super::*;

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Cow<'a, str>,
    #[serde(default)]
    pub meta: Value,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierPermissive<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Option<Cow<'a, str>>,
    #[serde(default)]
    pub meta: Value,
}

impl<'a> TryFrom<CibouletteResourceIdentifierPermissive<'a>> for CibouletteResourceIdentifier<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteResourceIdentifierPermissive<'a>) -> Result<Self, Self::Error> {
        let CibouletteResourceIdentifierPermissive { type_, id, meta } = value;

        Ok(CibouletteResourceIdentifier {
            type_,
            meta,
            id: id.ok_or(CibouletteError::MissingId)?,
        })
    }
}

impl<'a> From<CibouletteResourceIdentifier<'a>> for CibouletteResourceIdentifierPermissive<'a> {
    fn from(value: CibouletteResourceIdentifier<'a>) -> Self {
        let CibouletteResourceIdentifier { type_, id, meta } = value;

        CibouletteResourceIdentifierPermissive {
            type_,
            meta,
            id: Some(id),
        }
    }
}

impl<'a> CibouletteResourceIdentifier<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Cow<'a, str>, type_: Cow<'a, str>, meta: Value) -> Self {
        CibouletteResourceIdentifier { id, type_, meta }
    }
}

impl<'a> CibouletteResourceIdentifierPermissive<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<Cow<'a, str>>, type_: Cow<'a, str>, meta: Value) -> Self {
        CibouletteResourceIdentifierPermissive { id, type_, meta }
    }
}

/// ## A selector between a single or multiple `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) objects
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
}
