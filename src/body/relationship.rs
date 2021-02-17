use super::*;

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: Option<CibouletteResourceIdentifierSelector<'a>>,
    pub meta: HashMap<Cow<'a, str>, Value>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipBucket {
    resource: CibouletteResourceType,
    from: String,
    to: String,
}

impl CibouletteRelationshipBucket {
    pub fn new(resource: CibouletteResourceType, from: String, to: String) -> Self {
        CibouletteRelationshipBucket { resource, from, to }
    }
}

#[derive(Debug, Clone)]
pub enum CibouletteRelationshipOption {
    /// One to one relationship, boolean if the relationship is optional
    One(bool),
    /// One to many relationship
    Many(CibouletteRelationshipBucket),
}
