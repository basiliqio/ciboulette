use super::*;
use getset::CopyGetters;

mod builder;
mod many_to_many;
mod one_to_many;
mod relationships_options;
mod relationships_options_builder;
pub use builder::CibouletteStoreBuilder;
pub use relationships_options::{
    CibouletteRelationshipManyToManyOption, CibouletteRelationshipOneToManyOption,
    CibouletteRelationshipOption,
};
pub use relationships_options_builder::{
    CibouletteRelationshipManyToManyOptionBuilder, CibouletteRelationshipOneToManyOptionBuilder,
    CibouletteRelationshipOptionBuilder,
};

#[cfg(test)]
mod tests;

/// ## Map of accepted resource types
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut)]
pub struct CibouletteStore {
    graph: petgraph::graph::Graph<
        Arc<CibouletteResourceType>,
        CibouletteRelationshipOption,
        petgraph::Directed,
        u16,
    >,
    map: BTreeMap<ArcStr, petgraph::graph::NodeIndex<u16>>,
    #[getset(get_mut = "pub")]
    config: CibouletteConfig,
}

impl CibouletteStore {
    /// Get a type index from the graph
    pub fn get_type_index(&self, name: &str) -> Option<&petgraph::graph::NodeIndex<u16>> {
        self.map.get(name)
    }

    /// Get a type from the graph
    pub fn get_type_with_index(
        &self,
        name: &str,
    ) -> Option<(
        petgraph::graph::NodeIndex<u16>,
        &Arc<CibouletteResourceType>,
    )> {
        self.map
            .get(name)
            .and_then(|x| self.graph.node_weight(*x).map(|y| (*x, y)))
    }

    /// Get a type from the graph, returning an error if not found
    pub fn get_type_if_exists(&self, name: &str) -> Option<Arc<CibouletteResourceType>> {
        self.map
            .get(name)
            .and_then(|x| self.graph.node_weight(*x))
            .cloned()
    }

    /// Get a type from the graph, returning an error if not found
    pub fn get_type(&self, name: &str) -> Result<&Arc<CibouletteResourceType>, CibouletteError> {
        self.map
            .get(name)
            .and_then(|x| self.graph.node_weight(*x))
            .ok_or_else(|| CibouletteError::UnknownType(name.to_string()))
    }

    /// Get a relationship from the graph
    pub fn get_rel(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(&CibouletteResourceType, &CibouletteRelationshipOption), CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?;
        let from_type = self
            .graph
            .node_weight(*from_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(from.to_string()))?;
        let rel = from_type.relationships().get(to).ok_or_else(|| {
            CibouletteError::UnknownRelationship(from.to_string(), to.to_string())
        })?;
        let (_from_type_i, to_type_i) = self
            .graph
            .edge_endpoints(*rel)
            .ok_or_else(|| CibouletteError::RelNotInGraph(from.to_string(), to.to_string()))?;
        let to_type = self
            .graph
            .node_weight(to_type_i)
            .ok_or_else(|| CibouletteError::RelNotInGraph(from.to_string(), to.to_string()))?;
        let opt = self
            .graph
            .edge_weight(*rel)
            .ok_or_else(|| CibouletteError::RelNotInGraph(from.to_string(), to.to_string()))?;

        Ok((to_type, opt))
    }
}
