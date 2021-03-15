use super::*;

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>>,
    pub meta: HashMap<Cow<'a, str>, Value>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipBucket<'a> {
    resource: CibouletteResourceType<'a>,
    from: String,
    to: String,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToOneOption {
    key: String,
    id_type: CibouletteIdType,
    optional: bool,
}

impl CibouletteRelationshipOneToOneOption {
    pub fn new(key: String, id_type: CibouletteIdType, optional: bool) -> Self {
        CibouletteRelationshipOneToOneOption {
            key,
            id_type,
            optional,
        }
    }
}

impl<'a> CibouletteRelationshipBucket<'a> {
    pub fn new(resource: CibouletteResourceType<'a>, from: String, to: String) -> Self {
        CibouletteRelationshipBucket { resource, from, to }
    }
}

impl<'a> Default for CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>> {
    fn default() -> Self {
        CibouletteOptionalData::Null(false)
    }
}

#[derive(Debug, Clone)]
pub enum CibouletteRelationshipOption<'a> {
    /// One to one relationship, boolean if the relationship is optional
    One(CibouletteRelationshipOneToOneOption),
    /// One to many relationship, without the intermediate node
    ManyDirect(CibouletteRelationshipBucket<'a>),
    /// One to many relationship
    Many(CibouletteRelationshipBucket<'a>),
}
