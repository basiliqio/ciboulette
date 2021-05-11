use super::*;

/// ## Many-to-Many relationships options
#[derive(Debug, Clone, Getters, Ord, PartialEq, PartialOrd, Eq, Hash)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOption {
    /// The bucket resource
    pub(crate) bucket_resource: Arc<CibouletteResourceType>,
    /// The related types and their relating fields
    pub(crate) keys: [(Arc<CibouletteResourceType>, ArcStr); 2],
}

/// ## One-to-Many/Many-to-One relationships options
#[derive(Debug, Clone, Getters, Ord, PartialEq, PartialOrd, Eq, Hash)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOption {
    /// The "one" resource
    pub(crate) one_resource: Arc<CibouletteResourceType>,
    /// The field in the "one" resource that points to the "many" resource
    pub(crate) one_resource_key: ArcStr,
    /// The "many" resource
    pub(crate) many_resource: Arc<CibouletteResourceType>,
    /// The "many" field relating to the "one" resource
    pub(crate) many_resource_key: ArcStr,
    /// True if the relationships is optional
    pub(crate) optional: bool,
    /// Contains the relationship edge is part of many-to-many relationships
    pub(crate) part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
}

impl CibouletteRelationshipManyToManyOption {
    /// Get the relating field in the bucket resource for the provided type
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
    /// Build into a new [CibouletteRelationshipObject](CibouletteRelationshipObject)
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

/// ## Relationships options
#[derive(Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
pub enum CibouletteRelationshipOption {
    /// One to many relationship, without the intermediate node
    OneToMany(Arc<CibouletteRelationshipOneToManyOption>),
    /// One to many relationship, without the intermediate node
    ManyToOne(Arc<CibouletteRelationshipOneToManyOption>),
    /// One to many relationship
    ManyToMany(Arc<CibouletteRelationshipManyToManyOption>),
}
