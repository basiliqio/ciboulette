use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug)]
pub struct CibouletteBag {
    graph: petgraph::graph::Graph<CibouletteResourceType, bool, petgraph::Directed, u16>,
    map: BTreeMap<String, petgraph::graph::NodeIndex<u16>>,
}

impl<'a> CibouletteBag {
    /// Create a new bag
    pub fn new() -> Self {
        CibouletteBag {
            graph: petgraph::graph::Graph::with_capacity(0, 0),
            map: BTreeMap::new(),
        }
    }

    /// Get the inner map
    pub fn map(&self) -> &BTreeMap<String, petgraph::graph::NodeIndex<u16>> {
        &self.map
    }

    /// Get the inner graph
    pub fn graph(
        &self,
    ) -> &petgraph::graph::Graph<CibouletteResourceType, bool, petgraph::Directed, u16> {
        &self.graph
    }

    /// Get a type from the graph
    pub fn get_type(&'a self, name: &str) -> Option<&'a CibouletteResourceType> {
        self.map.get(name).and_then(|x| self.graph.node_weight(*x))
    }

    /// Get a relationship from the graph
    pub fn get_rel(
        &'a self,
        from: &str,
        to: &str,
    ) -> Result<(&'a CibouletteResourceType, bool), CibouletteError> {
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
            .ok_or_else(|| CibouletteError::RelNotInGraph(to.to_string()))?;
        let to_type = self
            .graph
            .node_weight(to_type_i)
            .ok_or_else(|| CibouletteError::RelNotInGraph(to.to_string()))?;
        let optional = self
            .graph
            .edge_weight(*rel)
            .ok_or_else(|| CibouletteError::RelNotInGraph(to.to_string()))?;

        Ok((to_type, *optional))
    }

    /// Add a type to the graph
    pub fn add_type(&mut self, name: String, schema: MessyJson) -> Result<(), CibouletteError> {
        if self.map.contains_key(name.as_str())
        // Check if type exists
        {
            return Err(CibouletteError::UniqType(name));
        }

        let t = CibouletteResourceType::new(name.clone(), schema);
        let index = self.graph.add_node(t); // Add the node
        self.map.insert(name, index); // Save the index to the map
        Ok(())
    }

    /// Add a relationships to the graph
    pub fn add_rel(
        &mut self,
        from: &str,
        to: &str,
        alias: Option<&str>,
        optional: bool,
    ) -> Result<(), CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?;
        let to_i = self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?;
        let edge_i = self.graph.update_edge(*from_i, *to_i, optional);
        let type_ = self
            .graph
            .node_weight_mut(*from_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(from.to_string()))?;
        let alias = alias.unwrap_or(to);
        if type_.relationships().contains_key(alias) {
            self.graph.remove_edge(edge_i); // Cancel the created edge
            return Err(CibouletteError::UniqRelationship(
                from.to_string(),
                alias.to_string(),
            ));
        }
        type_.relationships_mut().insert(alias.to_string(), edge_i);
        Ok(())
    }
}
