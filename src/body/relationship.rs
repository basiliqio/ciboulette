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

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOption<'a> {
    bucket_resource: CibouletteResourceType<'a>,
    keys: [(CibouletteResourceType<'a>, String); 2],
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOption<'a> {
    one_table: CibouletteResourceType<'a>,
    many_table: CibouletteResourceType<'a>,
    many_table_key: String,
    optional: bool,
    part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToOneOption {
    key: String,
    id_type: CibouletteIdType,
    optional: bool,
}

impl CibouletteRelationshipOneToOneOption {
    pub fn new(key: &str, id_type: CibouletteIdType, optional: bool) -> Self {
        CibouletteRelationshipOneToOneOption {
            key: key.to_string(),
            id_type,
            optional,
        }
    }
}

impl<'a> CibouletteRelationshipManyToManyOption<'a> {
    pub fn new(
        bucket_resource: CibouletteResourceType<'a>,
        keys: [(CibouletteResourceType<'a>, String); 2],
    ) -> Self {
        CibouletteRelationshipManyToManyOption {
            bucket_resource,
            keys,
        }
    }

    pub fn keys_for_type(
        &self,
        type_: &CibouletteResourceType<'a>,
    ) -> Result<&str, CibouletteError> {
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

impl<'a> CibouletteRelationshipOneToManyOption<'a> {
    pub fn new(
        one_table: CibouletteResourceType<'a>,
        many_table: CibouletteResourceType<'a>,
        many_table_key: String,
        optional: bool,
    ) -> Self {
        CibouletteRelationshipOneToManyOption {
            one_table,
            many_table,
            many_table_key,
            part_of_many_to_many: None,
            optional,
        }
    }

    pub(crate) fn new_from_many_to_many(
        one_table: CibouletteResourceType<'a>,
        many_table: CibouletteResourceType<'a>,
        many_table_key: String,
        optional: bool,
        part_of_many_to_many: petgraph::graph::EdgeIndex<u16>,
    ) -> Self {
        CibouletteRelationshipOneToManyOption {
            one_table,
            many_table,
            many_table_key,
            part_of_many_to_many: Some(part_of_many_to_many),
            optional,
        }
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
    OneToOne(CibouletteRelationshipOneToOneOption),
    /// One to many relationship, without the intermediate node
    OneToMany(CibouletteRelationshipOneToManyOption<'a>),
    /// One to many relationship, without the intermediate node
    ManyToOne(CibouletteRelationshipOneToManyOption<'a>),
    /// One to many relationship
    ManyToMany(CibouletteRelationshipManyToManyOption<'a>),
}
