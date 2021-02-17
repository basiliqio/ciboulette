use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug)]
pub struct CibouletteStore {
    graph: petgraph::graph::Graph<
        CibouletteResourceType,
        CibouletteRelationshipOption,
        petgraph::Directed,
        u16,
    >,
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
    ) -> &petgraph::graph::Graph<
        CibouletteResourceType,
        CibouletteRelationshipOption,
        petgraph::Directed,
        u16,
    > {
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
    ) -> Result<(&'a CibouletteResourceType, &'a CibouletteRelationshipOption), CibouletteError>
    {
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
        let opt = self
            .graph
            .edge_weight(*rel)
            .ok_or_else(|| CibouletteError::RelNotInGraph(to.to_string()))?;

        Ok((to_type, opt))
    }

    /// Add a type to the graph
    pub fn add_type(&mut self, name: String, schema: MessyJson) -> Result<(), CibouletteError> {
        if self.map.contains_key(name.as_str())
        // Check if type exists
        {
            return Err(CibouletteError::UniqType(name));
        }
        if let MessyJson::Obj(_) = schema {
            let t = CibouletteResourceType::new(name.clone(), schema);
            let index = self.graph.add_node(t); // Add the node
            self.map.insert(name, index); // Save the index to the map
            Ok(())
        } else {
            Err(CibouletteError::AttributesIsNotAnObject)
        }
    }

    fn check_type_has_fields(
        &'a self,
        type_: &'a CibouletteResourceType,
        fields: &'a [&'a str],
    ) -> Result<Option<&str>, CibouletteError> {
        match type_.schema() {
            MessyJson::Obj(obj) => {
                Ok(fields
                    .iter()
                    .find_map(|k| match obj.properties().contains_key(*k) {
                        true => None,
                        false => Some(*k),
                    }))
            }
            _ => Err(CibouletteError::AttributesIsNotAnObject),
        }
    }

    /// Add a relationships to the graph
    pub fn add_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipOption,
    ) -> Result<(), CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?; // Get `from` index
        let to_i = self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?; // Get `to index
        if let CibouletteRelationshipOption::Many(opt) = &opt {
            // If a one-to-many relationship
            let type_fetched = self
                .map()
                .get(opt.resource().name())
                .and_then(|x| self.graph.node_weight(*x)); // Check the bucket type exists
            match type_fetched {
                None => return Err(CibouletteError::TypeNotInGraph(from.to_string())), // If it doens't, its an error
                Some(x) if x != opt.resource() => {
                    return Err(CibouletteError::TypeNotInGraph(from.to_string()));
                    // If it exists but types aren't equals, it's also an error
                }
                Some(x) => {
                    let fields: [&str; 2] = [opt.from().as_str(), opt.to().as_str()];
                    if let Some(missing) = self.check_type_has_fields(x, &fields)? {
                        return Err(CibouletteError::UnknownField(
                            opt.resource().name().clone(),
                            missing.to_string(),
                        ));
                    }
                }
            }
        }
        let edge_i = self.graph.update_edge(*from_i, *to_i, opt); // Get the edge index
        let edge_i_reverse =
            self.graph
                .update_edge(*to_i, *from_i, CibouletteRelationshipOption::One(false)); // Get the edge index
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
            type_
                .relationships_type_to_alias_mut()
                .insert(to.to_string(), alias.to_string()); // And the translation between type_ and alias
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
            type_
                .relationships_type_to_alias_mut()
                .insert(to.to_string(), alias.to_string()); // And the translation between type_ and alias
        }
        Ok(())
    }
}
