use super::*;

#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObjectBuilder<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelectorBuilder<'a>>,
    pub meta: Value,
}

/// ## A `json:api` [relationship](https://jsonapi.org/format/#document-resource-object-relationships) object
#[derive(Debug, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteRelationshipObject<'a> {
    pub links: Option<CibouletteLink<'a>>,
    pub data: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>>,
    pub meta: Value,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipBucket<'a> {
    bucket_resource: CibouletteResourceType<'a>,
    keys: [(CibouletteResourceType<'a>, String); 2],
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
    pub fn new(
        bucket_resource: CibouletteResourceType<'a>,
        keys: [(CibouletteResourceType<'a>, String); 2],
    ) -> Self {
        CibouletteRelationshipBucket {
            bucket_resource,
            keys,
        }
    }

    pub fn keys_for_type(
        &'a self,
        type_: &'a CibouletteResourceType<'a>,
    ) -> Result<&'a str, CibouletteError> {
        self.keys
            .iter()
            .find(|(k, _)| k == type_)
            .map(|x| x.1.as_str())
            .ok_or_else(|| {
                CibouletteError::UnknownRelationship(
                    self.bucket_resource().name().clone(),
                    type_.name().clone(),
                )
            })
    }
}

impl<'a> Default for CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>> {
    fn default() -> Self {
        CibouletteOptionalData::Null(false)
    }
}

impl<'a> CibouletteRelationshipObjectBuilder<'a> {
    pub fn build(
        self,
        type_: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteRelationshipObject<'a>, CibouletteError> {
        Ok(CibouletteRelationshipObject {
            links: self.links,
            meta: self.meta,
            data: match self.data {
                CibouletteOptionalData::Null(x) => CibouletteOptionalData::Null(x),
                CibouletteOptionalData::Object(obj) => {
                    CibouletteOptionalData::Object(obj.build(&type_)?)
                }
            },
        })
    }
}

impl<'a> Default for CibouletteOptionalData<CibouletteResourceIdentifierSelectorBuilder<'a>> {
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
