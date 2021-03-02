use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut)]
pub struct CibouletteStore<'a> {
    graph: petgraph::graph::Graph<
        CibouletteResourceType<'a>,
        CibouletteRelationshipOption<'a>,
        petgraph::Directed,
        u16,
    >,
    map: BTreeMap<String, petgraph::graph::NodeIndex<u16>>,
}

impl<'a> Default for CibouletteStore<'a> {
    #[inline]
    fn default() -> Self {
        CibouletteStore {
            graph: petgraph::graph::Graph::with_capacity(0, 0),
            map: BTreeMap::new(),
        }
    }
}

impl<'a> CibouletteStore<'a> {
    /// Create a new bag
    #[inline]
    pub fn new() -> Self {
        CibouletteStore::default()
    }

    /// Get a type index from the graph
    pub fn get_type_index(&self, name: &str) -> Option<&petgraph::graph::NodeIndex<u16>> {
        self.map.get(name)
    }

    /// Get a type from the graph
    pub fn get_type_with_index(
        &self,
        name: &str,
    ) -> Option<(petgraph::graph::NodeIndex<u16>, &CibouletteResourceType<'a>)> {
        self.map
            .get(name)
            .and_then(|x| self.graph.node_weight(*x).map(|y| (*x, y)))
    }

    /// Get a type from the graph
    pub fn get_type(&self, name: &str) -> Option<&CibouletteResourceType<'a>> {
        self.map.get(name).and_then(|x| self.graph.node_weight(*x))
    }

    /// Get a relationship from the graph
    pub fn get_rel(
        &self,
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

    /// Add a type to the graph
    pub fn add_type(
        &mut self,
        name: String,
        schema: MessyJsonObject<'a>,
    ) -> Result<(), CibouletteError> {
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

    /// Add a relationships (one-to-one) to the graph
    pub fn add_rel_single(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipOneToOneOption,
    ) -> Result<(), CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?; // Get `from` index
        let to_i = self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?;
        let edge_i = self
            .graph
            .update_edge(*from_i, *to_i, CibouletteRelationshipOption::One(opt)); // Get the edge index
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
                return Err(CibouletteError::UniqRelationship(
                    to.to_string(),
                    alias.to_string(),
                ));
            }
            type_.relationships_mut().insert(alias.to_string(), edge_i); // Insert the relationship
            type_
                .relationships_type_to_alias_mut()
                .insert(from.to_string(), alias.to_string()); // And the translation between type_ and alias
        }
        Ok(())
    }

    // fn check_type_has_fields(&self, type_: petgraph::graph::NodeIndex<u16>, opt: &CibouletteRelationshipBucket<'a>) -> Result<(), CibouletteError>
    // {
    // 	let type_fetched = self.graph_mut().node_weight_mut(type_); // Check the bucket type exists
    // 	match type_fetched {
    // 		None => return Err(CibouletteError::TypeNotInGraph(from.to_string())), // If it doens't, its an error
    // 		Some(x) if x != opt.resource() => {
    // 			return Err(CibouletteError::TypeNotInGraph(from.to_string()));
    // 			// If it exists but types aren't equals, it's also an error
    // 		}
    // 		Some(x) => {
    // 			let fields: [&str; 2] = [opt.from().as_str(), opt.to().as_str()];
    // 			if let Some(missing) = x.has_fields(&fields)? {
    // 				return Err(CibouletteError::UnknownField(
    // 					opt.resource().name().clone(),
    // 					missing,
    // 				));
    // 			}
    // 		}
    // 	};
    // }

    /// Add a relationships (one/many-to-one/many) to the graph
    pub fn add_rel_multiple(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipBucket<'a>,
    ) -> Result<(), CibouletteError> {
        let (from_i, to_i, bucket_i) = {
            let from_i = self
                .map
                .get(from)
                .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?; // Get `from` index
            let to_i = self
                .map
                .get(to)
                .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?; // Get `to index
            let bucket_i = self
                .map
                .get(opt.resource().name())
                .ok_or_else(|| CibouletteError::UnknownType(opt.resource().name().clone()))?; // Get `to index
            (*from_i, *to_i, *bucket_i)
        };
        {
            let type_fetched = self.graph.node_weight(bucket_i); // Check the bucket type exists
            match type_fetched {
                None => return Err(CibouletteError::TypeNotInGraph(from.to_string())), // If it doens't, its an error
                Some(x) if x != opt.resource() => {
                    return Err(CibouletteError::TypeNotInGraph(from.to_string()));
                    // If it exists but types aren't equals, it's also an error
                }
                Some(x) => {
                    let fields: [&str; 2] = [opt.from().as_str(), opt.to().as_str()];
                    if let Some(missing) = x.has_fields(&fields)? {
                        return Err(CibouletteError::UnknownField(
                            opt.resource().name().clone(),
                            missing,
                        ));
                    }
                }
            };
        };
        let edge_from_i = self.graph_mut().update_edge(
            bucket_i,
            from_i,
            CibouletteRelationshipOption::Many(opt.clone()),
        ); // Get the edge index
        let edge_to_i = self.graph_mut().update_edge(
            bucket_i,
            to_i,
            CibouletteRelationshipOption::Many(opt.clone()),
        ); // Get the edge index
        let edge_from_i_direct = self.graph_mut().update_edge(
            from_i,
            to_i,
            CibouletteRelationshipOption::ManyDirect(opt.clone()),
        ); // Add the direct edges
        let edge_to_i_direct = self.graph_mut().update_edge(
            to_i,
            from_i,
            CibouletteRelationshipOption::ManyDirect(opt.clone()),
        ); // Add the direct edges
        {
            // Handle edge
            let type_ = self
                .graph
                .node_weight_mut(from_i)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(from.to_string()))?; // Get the type
            let alias = alias_to.unwrap_or(to); // Override if there is no alias
            if type_.relationships().contains_key(alias) {
                // Check if relationship exists
                self.graph.remove_edge(edge_from_i); // Cancel the created edge
                self.graph.remove_edge(edge_to_i);
                self.graph.remove_edge(edge_from_i_direct);
                self.graph.remove_edge(edge_to_i_direct);
                return Err(CibouletteError::UniqRelationship(
                    from.to_string(),
                    alias.to_string(),
                ));
            }
            type_
                .relationships_mut()
                .insert(alias.to_string(), edge_to_i_direct); // Insert the relationship
            type_
                .relationships_type_to_alias_mut()
                .insert(to.to_string(), alias.to_string()); // And the translation between type_ and alias
        }
        {
            // Handle reverse edge
            let type_ = self
                .graph
                .node_weight_mut(to_i)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(to.to_string()))?; // Get the type
            let alias = alias_from.unwrap_or(from); // Override if there is no alias
            if type_.relationships().contains_key(alias) {
                // Check if relationship exists
                self.graph.remove_edge(edge_from_i); // Cancel the created edge
                self.graph.remove_edge(edge_to_i);
                self.graph.remove_edge(edge_from_i_direct);
                self.graph.remove_edge(edge_to_i_direct);
                return Err(CibouletteError::UniqRelationship(
                    to.to_string(),
                    alias.to_string(),
                ));
            }
            type_
                .relationships_mut()
                .insert(alias.to_string(), edge_from_i_direct); // Insert the relationship
            type_
                .relationships_type_to_alias_mut()
                .insert(from.to_string(), alias.to_string()); // And the translation between type_ and alias
        }
        Ok(())
    }
}
