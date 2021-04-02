use super::*;

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipManyToManyOptionBuilder<'a> {
    bucket_resource: CibouletteResourceType<'a>,
    keys: [(CibouletteResourceType<'a>, String); 2],
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CibouletteRelationshipOneToManyOptionBuilder<'a> {
    one_table: CibouletteResourceType<'a>,
    many_table: CibouletteResourceType<'a>,
    many_table_key: String,
    optional: bool,
    part_of_many_to_many: Option<petgraph::graph::EdgeIndex<u16>>,
}

impl<'a> CibouletteRelationshipManyToManyOptionBuilder<'a> {
    pub fn new(
        bucket_resource: CibouletteResourceType<'a>,
        keys: [(CibouletteResourceType<'a>, String); 2],
    ) -> Self {
        CibouletteRelationshipManyToManyOptionBuilder {
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
    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder<'a>,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType<'a>>,
            CibouletteRelationshipOption<'a>,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<CibouletteRelationshipManyToManyOption<'a>, CibouletteError> {
        let bucket_table = store_builder
            .get_type_index(self.bucket_resource().name())
            .ok_or_else(|| {
                CibouletteError::TypeNotInGraph(self.bucket_resource().name().clone())
            })?;
        let table1 = store_builder
            .get_type_index(self.keys()[0].0.name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.keys()[0].0.name().clone()))?;
        let table2 = store_builder
            .get_type_index(self.keys()[1].0.name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.keys()[1].0.name().clone()))?;

        Ok(CibouletteRelationshipManyToManyOption {
            bucket_resource: graph
                .node_weight(*bucket_table)
                .ok_or_else(|| {
                    CibouletteError::TypeNotInGraph(self.bucket_resource().name().clone())
                })?
                .clone(),
            keys: [
                (
                    graph
                        .node_weight(*table1)
                        .ok_or_else(|| {
                            CibouletteError::TypeNotInGraph(self.keys()[0].0.name().clone())
                        })?
                        .clone(),
                    self.keys[0].1.clone(),
                ),
                (
                    graph
                        .node_weight(*table2)
                        .ok_or_else(|| {
                            CibouletteError::TypeNotInGraph(self.keys()[1].0.name().clone())
                        })?
                        .clone(),
                    self.keys[1].1.clone(),
                ),
            ],
        })
    }
}

impl<'a> CibouletteRelationshipOneToManyOptionBuilder<'a> {
    pub fn new(
        one_table: CibouletteResourceType<'a>,
        many_table: CibouletteResourceType<'a>,
        many_table_key: String,
        optional: bool,
    ) -> Self {
        CibouletteRelationshipOneToManyOptionBuilder {
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
        CibouletteRelationshipOneToManyOptionBuilder {
            one_table,
            many_table,
            many_table_key,
            part_of_many_to_many: Some(part_of_many_to_many),
            optional,
        }
    }

    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder<'a>,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType<'a>>,
            CibouletteRelationshipOption<'a>,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<CibouletteRelationshipOneToManyOption<'a>, CibouletteError> {
        let one_table = store_builder
            .get_type_index(self.one_table().name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.one_table().name().clone()))?;
        let many_table = store_builder
            .get_type_index(self.many_table().name())
            .ok_or_else(|| CibouletteError::TypeNotInGraph(self.many_table().name().clone()))?;

        Ok(CibouletteRelationshipOneToManyOption {
            one_table: graph
                .node_weight(*one_table)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(self.one_table().name().clone()))?
                .clone(),
            many_table: graph
                .node_weight(*many_table)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(self.many_table().name().clone()))?
                .clone(),
            many_table_key: self.many_table_key.clone(),
            optional: self.optional,
            part_of_many_to_many: self.part_of_many_to_many,
        })
    }
}

#[derive(Debug, Clone)]
pub enum CibouletteRelationshipOptionBuilder<'a> {
    /// One to many relationship, without the intermediate node
    OneToMany(CibouletteRelationshipOneToManyOptionBuilder<'a>),
    /// One to many relationship, without the intermediate node
    ManyToOne(CibouletteRelationshipOneToManyOptionBuilder<'a>),
    /// One to many relationship
    ManyToMany(CibouletteRelationshipManyToManyOptionBuilder<'a>),
}

impl<'a> CibouletteRelationshipOptionBuilder<'a> {
    pub(crate) fn build(
        &self,
        store_builder: &CibouletteStoreBuilder<'a>,
        graph: &petgraph::graph::Graph<
            Arc<CibouletteResourceType<'a>>,
            CibouletteRelationshipOption<'a>,
            petgraph::Directed,
            u16,
        >,
    ) -> Result<CibouletteRelationshipOption<'a>, CibouletteError> {
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
