use super::*;

#[derive(Debug, Clone, Getters, Ord, PartialEq, PartialOrd, Eq, Hash)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOption {
    pub(crate) bucket_resource: Arc<CibouletteResourceType>,
    pub(crate) keys: [(Arc<CibouletteResourceType>, ArcStr); 2],
}

#[derive(Debug, Clone, Getters, Ord, PartialEq, PartialOrd, Eq, Hash)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOption {
    pub(crate) one_table: Arc<CibouletteResourceType>,
    pub(crate) many_table: Arc<CibouletteResourceType>,
    pub(crate) many_table_key: ArcStr,
    pub(crate) optional: bool,
    pub(crate) part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToOneOption {
    key: ArcStr,
    id_type: CibouletteIdType,
    optional: bool,
}

impl CibouletteRelationshipOneToOneOption {
    pub fn new(key: &str, id_type: CibouletteIdType, optional: bool) -> Self {
        CibouletteRelationshipOneToOneOption {
            key: ArcStr::from(key),
            id_type,
            optional,
        }
    }
}

impl CibouletteRelationshipManyToManyOption {
    pub fn keys_for_type(&self, type_: &CibouletteResourceType) -> Result<ArcStr, CibouletteError> {
        self.keys
            .iter()
            .find(|(k, _)| k.as_ref() == type_)
            .map(|x| x.1.clone())
            .ok_or_else(|| {
                CibouletteError::UnknownRelationship(
                    self.bucket_resource().name().to_string(),
                    type_.name().to_string(),
                )
            })
    }
}

impl CibouletteRelationshipOneToManyOption {}

impl<'request> CibouletteRelationshipObjectBuilder<'request> {
    pub fn build(
        self,
        type_: &Arc<CibouletteResourceType>,
    ) -> Result<CibouletteRelationshipObject<'request>, CibouletteError> {
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
#[derive(Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
pub enum CibouletteRelationshipOption {
    /// One to many relationship, without the intermediate node
    OneToMany(Arc<CibouletteRelationshipOneToManyOption>),
    /// One to many relationship, without the intermediate node
    ManyToOne(Arc<CibouletteRelationshipOneToManyOption>),
    /// One to many relationship
    ManyToMany(Arc<CibouletteRelationshipManyToManyOption>),
}
