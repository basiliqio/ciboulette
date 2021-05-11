use super::*;

/// ## Many-to-Many relationships option builder
#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOptionBuilder {
    /// The bucket resource
    bucket_resource: CibouletteResourceType,
    /// The keys associated type and their matching field in bucket
    keys: [(CibouletteResourceType, ArcStr); 2],
}

/// ## One-to-Many/Many-to-one relationships option builder
#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOptionBuilder {
    /// The "one" resource
    one_resource: CibouletteResourceType,
    /// The field in the "one" resource that points to the "many" resource
    one_resource_key: ArcStr,
    /// The "many" resource
    many_resource: CibouletteResourceType,
    /// The field in the "many" resource that points to the "one" resource
    many_resource_key: ArcStr,
    /// True if that relationships is optional
    optional: bool,
    /// True if this relationships is part of Many-to-Many relationships
    part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
}

impl CibouletteRelationshipManyToManyOptionBuilder {
    pub fn new(
        bucket_resource: CibouletteResourceType,
        keys: [(CibouletteResourceType, ArcStr); 2],
    ) -> Self {
        CibouletteRelationshipManyToManyOptionBuilder {
            bucket_resource,
            keys,
        }
    }

    /// Get the field for the resource in a Many-to-Many relationships
    pub fn keys_for_type(&self, type_: &CibouletteResourceType) -> Result<ArcStr, CibouletteError> {
        self.keys
            .iter()
            .find(|(k, _)| k == type_)
            .map(|x| x.1.clone())
            .ok_or_else(|| {
                CibouletteError::UnknownRelationship(
                    self.bucket_resource().name().to_string(),
                    type_.name().to_string(),
                )
            })
    }

    /// Build into [Arc<CibouletteRelationshipManyToManyOption>](CibouletteRelationshipManyToManyOption)
    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType>,
            CibouletteRelationshipOption,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<Arc<CibouletteRelationshipManyToManyOption>, CibouletteError> {
        let bucket_table = store_builder
            .get_type_index(self.bucket_resource().name())
            .ok_or_else(|| {
                CibouletteError::TypeNotInGraph(self.bucket_resource().name().to_string())
            })?;
        let table1 = store_builder
            .get_type_index(self.keys()[0].0.name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.keys()[0].0.name().to_string()))?;
        let table2 = store_builder
            .get_type_index(self.keys()[1].0.name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.keys()[1].0.name().to_string()))?;

        Ok(Arc::from(CibouletteRelationshipManyToManyOption {
            bucket_resource: graph
                .node_weight(*bucket_table)
                .ok_or_else(|| {
                    CibouletteError::TypeNotInGraph(self.bucket_resource().name().to_string())
                })?
                .clone(),
            keys: [
                (
                    graph
                        .node_weight(*table1)
                        .ok_or_else(|| {
                            CibouletteError::TypeNotInGraph(self.keys()[0].0.name().to_string())
                        })?
                        .clone(),
                    self.keys[0].1.clone(),
                ),
                (
                    graph
                        .node_weight(*table2)
                        .ok_or_else(|| {
                            CibouletteError::TypeNotInGraph(self.keys()[1].0.name().to_string())
                        })?
                        .clone(),
                    self.keys[1].1.clone(),
                ),
            ],
        }))
    }
}

impl CibouletteRelationshipOneToManyOptionBuilder {
    pub fn new(
        one_resource: CibouletteResourceType,
        one_resource_key: ArcStr,
        many_resource: CibouletteResourceType,
        many_resource_key: ArcStr,
        optional: bool,
    ) -> Self {
        CibouletteRelationshipOneToManyOptionBuilder {
            one_resource,
            many_resource,
            one_resource_key,
            many_resource_key,
            part_of_many_to_many: None,
            optional,
        }
    }

    /// Build a new O2M/M2O relationships in the process of creating a new M2M relationships
    pub(crate) fn new_from_many_to_many(
        one_resource: CibouletteResourceType,
        one_resource_key: ArcStr,
        many_resource: CibouletteResourceType,
        many_resource_key: ArcStr,
        optional: bool,
        part_of_many_to_many: petgraph::graph::EdgeIndex<u16>,
    ) -> Self {
        CibouletteRelationshipOneToManyOptionBuilder {
            one_resource,
            one_resource_key,
            many_resource,
            many_resource_key,
            part_of_many_to_many: Some(part_of_many_to_many),
            optional,
        }
    }

    /// Build into [Arc<CibouletteRelationshipOneToManyOption>](CibouletteRelationshipOneToManyOption)
    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType>,
            CibouletteRelationshipOption,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<Arc<CibouletteRelationshipOneToManyOption>, CibouletteError> {
        let one_resource = store_builder
            .get_type_index(self.one_resource().name())
            .ok_or_else(|| {
                CibouletteError::TypeNotInGraph(self.one_resource().name().to_string())
            })?;
        let many_resource = store_builder
            .get_type_index(self.many_resource().name())
            .ok_or_else(|| {
                CibouletteError::TypeNotInGraph(self.many_resource().name().to_string())
            })?;

        Ok(Arc::from(CibouletteRelationshipOneToManyOption {
            one_resource: graph
                .node_weight(*one_resource)
                .ok_or_else(|| {
                    CibouletteError::TypeNotInGraph(self.one_resource().name().to_string())
                })?
                .clone(),
            many_resource: graph
                .node_weight(*many_resource)
                .ok_or_else(|| {
                    CibouletteError::TypeNotInGraph(self.many_resource().name().to_string())
                })?
                .clone(),
            many_resource_key: ArcStr::from(self.many_resource_key()),
            optional: self.optional,
            part_of_many_to_many: self.part_of_many_to_many,
        }))
    }
}

/// ## Relationship options builder
#[derive(Debug, Clone)]
pub enum CibouletteRelationshipOptionBuilder {
    /// One to many relationship, without the intermediate node
    OneToMany(CibouletteRelationshipOneToManyOptionBuilder),
    /// One to many relationship, without the intermediate node
    ManyToOne(CibouletteRelationshipOneToManyOptionBuilder),
    /// One to many relationship
    ManyToMany(CibouletteRelationshipManyToManyOptionBuilder),
}

impl CibouletteRelationshipOptionBuilder {
    /// Build into [CibouletteRelationshipOption](CibouletteRelationshipOption)
    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType>,
            CibouletteRelationshipOption,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<CibouletteRelationshipOption, CibouletteError> {
        match self {
            CibouletteRelationshipOptionBuilder::OneToMany(x) => Ok(
                CibouletteRelationshipOption::OneToMany(x.build(store_builder, graph)?),
            ),
            CibouletteRelationshipOptionBuilder::ManyToOne(x) => Ok(
                CibouletteRelationshipOption::ManyToOne(x.build(store_builder, graph)?),
            ),
            CibouletteRelationshipOptionBuilder::ManyToMany(x) => Ok(
                CibouletteRelationshipOption::ManyToMany(x.build(store_builder, graph)?),
            ),
        }
    }
}
