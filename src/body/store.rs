use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug)]
pub struct CibouletteStore {
    graph: petgraph::graph::Graph<CibouletteResourceType, bool, petgraph::Directed, u16>,
    map: BTreeMap<String, petgraph::graph::NodeIndex<u16>>,
}

impl Default for CibouletteStore {
    #[inline]
    fn default() -> Self {
        CibouletteStore {
            graph: petgraph::graph::Graph::with_capacity(0, 0),
            map: BTreeMap::new(),
        }
    }
}

impl<'a> CibouletteStore {
    /// Create a new bag
    #[inline]
    pub fn new() -> Self {
        CibouletteStore::default()
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
    pub fn get_type_with_index(
        &'a self,
        name: &str,
    ) -> Option<(petgraph::graph::NodeIndex<u16>, &'a CibouletteResourceType)> {
        self.map
            .get(name)
            .and_then(|x| self.graph.node_weight(*x).map(|y| (*x, y)))
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
        alias_from: Option<&str>,
        alias_to: Option<&str>,
        optional: bool,
    ) -> Result<(), CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?; // Get `from` index
        let to_i = self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?; // Get `to index
        let edge_i = self.graph.update_edge(*from_i, *to_i, optional); // Get the edge index
        let edge_i_reverse = self.graph.update_edge(*to_i, *from_i, false); // Get the edge index
        {
            // Handle edge
            let type_ = self
                .graph
                .node_weight_mut(*from_i)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(from.to_string()))?; // Get the type
            let alias = alias_to.unwrap_or(to); // Override if there is no alias
            if type_.relationships().contains_key(alias) {
                // Check if relationship exists
                self.graph.remove_edge(edge_i); // Cancel the created edge
                self.graph.remove_edge(edge_i_reverse); // Cancel the created edge (reverse)
                return Err(CibouletteError::UniqRelationship(
                    from.to_string(),
                    alias.to_string(),
                ));
            }
            type_.relationships_mut().insert(alias.to_string(), edge_i); // Insert the relationship
        }
        {
            // Handle reverse edge
            let type_ = self
                .graph
                .node_weight_mut(*to_i)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(to.to_string()))?; // Get the type
            let alias = alias_from.unwrap_or(from); // Override if there is no alias
            if type_.relationships().contains_key(alias) {
                // Check if relationship exists
                self.graph.remove_edge(edge_i); // Cancel the created edge
                self.graph.remove_edge(edge_i_reverse); // Cancel the created edge (reverse)
                return Err(CibouletteError::UniqRelationship(
                    to.to_string(),
                    alias.to_string(),
                ));
            }
            type_.relationships_mut().insert(alias.to_string(), edge_i); // Insert the relationship
        }
        Ok(())
    }
}
