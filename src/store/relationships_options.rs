use super::*;

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOption<'a> {
    pub(crate) bucket_resource: Arc<CibouletteResourceType<'a>>,
    pub(crate) keys: [(Arc<CibouletteResourceType<'a>>, String); 2],
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOption<'a> {
    pub(crate) one_table: Arc<CibouletteResourceType<'a>>,
    pub(crate) many_table: Arc<CibouletteResourceType<'a>>,
    pub(crate) many_table_key: String,
    pub(crate) optional: bool,
    pub(crate) part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
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
    pub fn keys_for_type(
        &self,
        type_: &Arc<CibouletteResourceType<'a>>,
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

impl<'a> CibouletteRelationshipOneToManyOption<'a> {}

impl<'a> Default for CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>> {
    fn default() -> Self {
        CibouletteOptionalData::Null(false)
    }
}

impl<'a> CibouletteRelationshipObjectBuilder<'a> {
    pub fn build(
        self,
        type_: &Arc<CibouletteResourceType<'a>>,
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
    /// One to many relationship, without the intermediate node
    OneToMany(CibouletteRelationshipOneToManyOption<'a>),
    /// One to many relationship, without the intermediate node
    ManyToOne(CibouletteRelationshipOneToManyOption<'a>),
    /// One to many relationship
    ManyToMany(CibouletteRelationshipManyToManyOption<'a>),
}
