use super::*;
use crate::body::relationship::CibouletteRelationshipOneToManyOption;
use getset::CopyGetters;

mod many_to_many;
mod one_to_many;
mod one_to_one;

#[derive(Clone, Debug, CopyGetters)]
#[getset(get_copy = "pub")]
struct CibouletteManyToManyNodeIndexes {
    from: petgraph::graph::NodeIndex<u16>,
    to: petgraph::graph::NodeIndex<u16>,
    bucket: petgraph::graph::NodeIndex<u16>,
}

#[derive(Clone, Debug, CopyGetters)]
#[getset(get_copy = "pub")]
struct CibouletteManyToManyEdgeIndexes {
    from: petgraph::graph::EdgeIndex<u16>,
    from_direct: petgraph::graph::EdgeIndex<u16>,
    to: petgraph::graph::EdgeIndex<u16>,
    to_direct: petgraph::graph::EdgeIndex<u16>,
}

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

    /// Get a type from the graph, returning an error if not found
    pub fn get_type_if_exists(&self, name: &str) -> Option<&CibouletteResourceType<'a>> {
        self.map.get(name).and_then(|x| self.graph.node_weight(*x))
    }

    /// Get a type from the graph, returning an error if not found
    pub fn get_type(&self, name: &str) -> Result<&CibouletteResourceType<'a>, CibouletteError> {
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
        id_type: CibouletteIdType,
        schema: MessyJsonObject<'a>,
    ) -> Result<(), CibouletteError> {
        if self.map.contains_key(name.as_str())
        // Check if type exists
        {
            return Err(CibouletteError::UniqType(name));
        }
        let t = CibouletteResourceType::new(name.clone(), id_type, schema);
        let index = self.graph.add_node(t); // Add the node
        self.map.insert(name, index); // Save the index to the map
        Ok(())
    }

    /// Add a relationships (one-to-one) to the graph
    pub fn add_one_to_one_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipOneToOneOption,
    ) -> Result<(), CibouletteError> {
        let from_i = *self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?; // Get `from` index
        let to_i = *self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?;
        let edge_i =
            self.graph
                .update_edge(from_i, to_i, CibouletteRelationshipOption::OneToOne(opt)); // Get the edge index
        self.add_one_to_one_rel_routine(from, (to, alias_to), &from_i, &edge_i)?;
        self.add_one_to_one_rel_routine(to, (from, alias_from), &to_i, &edge_i)?;
        Ok(())
    }

    fn add_one_to_one_rel_routine(
        &mut self,
        orig: &str,
        (dest, dest_alias): (&str, Option<&str>),
        orig_i: &petgraph::graph::NodeIndex<u16>,
        rel_i: &petgraph::graph::EdgeIndex<u16>,
    ) -> Result<(), CibouletteError> {
        let type_ = self
            .graph
            .node_weight_mut(*orig_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(orig.to_string()))?;
        let alias = dest_alias.unwrap_or(dest);
        if type_.relationships().contains_key(alias) {
            // Check if relationship exists
            self.graph.remove_edge(*rel_i); // Cancel the created edge
            return Err(CibouletteError::UniqRelationship(
                orig.to_string(),
                alias.to_string(),
            ));
        }
        type_.relationships_mut().insert(alias.to_string(), *rel_i);
        type_
            .relationships_type_to_alias_mut()
            .insert(dest.to_string(), alias.to_string());
        Ok(())
    }

    /// Add a relationships (one/many-to-one/many) to the graph
    pub fn add_many_to_many_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOption<'a>,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let edge_indexes = self.get_many_to_many_edge_indexes(&node_indexes, opt);
        self.add_many_to_many_rel_routine(
            (from, node_indexes.from()),
            (to, alias_to),
            &edge_indexes,
            edge_indexes.to_direct(),
        )?;
        self.add_many_to_many_rel_routine(
            (to, node_indexes.to()),
            (from, alias_from),
            &edge_indexes,
            edge_indexes.from_direct(),
        )?;
        Ok(())
    }

    fn add_many_to_many_rel_routine(
        &mut self,
        (orig, orig_i): (&str, petgraph::graph::NodeIndex<u16>),
        (dest, alias_dest): (&str, Option<&str>),
        edge_indexes: &CibouletteManyToManyEdgeIndexes,
        rel_to_insert: petgraph::graph::EdgeIndex<u16>,
    ) -> Result<(), CibouletteError> {
        let type_ = self
            .graph
            .node_weight_mut(orig_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(orig.to_string()))?;
        let alias = alias_dest.unwrap_or(dest);
        if type_.relationships().contains_key(alias) {
            // Check if relationship exists
            self.graph.remove_edge(edge_indexes.from()); // Cancel the created edge
            self.graph.remove_edge(edge_indexes.to());
            self.graph.remove_edge(edge_indexes.from_direct());
            self.graph.remove_edge(edge_indexes.to_direct());
            return Err(CibouletteError::UniqRelationship(
                orig.to_string(),
                alias.to_string(),
            ));
        }
        type_
            .relationships_mut()
            .insert(alias.to_string(), rel_to_insert);
        type_
            .relationships_type_to_alias_mut()
            .insert(dest.to_string(), alias.to_string());
        Ok(())
    }

    fn get_many_to_many_edge_indexes(
        &mut self,
        indexes: &CibouletteManyToManyNodeIndexes,
        opt: CibouletteRelationshipManyToManyOption<'a>,
    ) -> CibouletteManyToManyEdgeIndexes {
        let edge_from_i = self.graph_mut().update_edge(
            indexes.bucket(),
            indexes.from(),
            CibouletteRelationshipOption::ManyToMany(opt.clone()),
        );
        let edge_to_i = self.graph_mut().update_edge(
            indexes.bucket(),
            indexes.to(),
            CibouletteRelationshipOption::ManyToMany(opt.clone()),
        );
        let edge_from_i_direct = self.graph_mut().update_edge(
            indexes.from(),
            indexes.to(),
            CibouletteRelationshipOption::OneToMany(CibouletteRelationshipOneToManyOption::from(
                &opt,
            )),
        );
        let edge_to_i_direct = self.graph_mut().update_edge(
            indexes.to(),
            indexes.from(),
            CibouletteRelationshipOption::OneToMany(CibouletteRelationshipOneToManyOption::from(
                &opt,
            )),
        );
        CibouletteManyToManyEdgeIndexes {
            from: edge_from_i,
            to: edge_to_i,
            from_direct: edge_from_i_direct,
            to_direct: edge_to_i_direct,
        }
    }

    fn check_bucket_exists(
        &mut self,
        bucket_i: petgraph::graph::NodeIndex<u16>,
        from: &str,
        opt: &CibouletteRelationshipManyToManyOption<'a>,
    ) -> Result<(), CibouletteError> {
        let type_fetched = self.graph.node_weight(bucket_i);
        match type_fetched {
            None => return Err(CibouletteError::TypeNotInGraph(from.to_string())), // If it doens't, its an error
            Some(x) if x != opt.bucket_resource() => {
                return Err(CibouletteError::TypeNotInGraph(from.to_string()));
                // If it exists but types aren't equals, it's also an error
            }
            Some(x) => {
                if let Some(missing) = x.has_fields(opt.keys().iter().map(|x| x.1.as_str()))? {
                    return Err(CibouletteError::UnknownField(
                        opt.bucket_resource().name().clone(),
                        missing,
                    ));
                }
            }
        };
        Ok(())
    }

    fn get_many_to_many_node_indexes(
        &mut self,
        from: &str,
        to: &str,
        opt: &CibouletteRelationshipManyToManyOption,
    ) -> Result<CibouletteManyToManyNodeIndexes, CibouletteError> {
        let from_i = self
            .map
            .get(from)
            .ok_or_else(|| CibouletteError::UnknownType(from.to_string()))?;
        let to_i = self
            .map
            .get(to)
            .ok_or_else(|| CibouletteError::UnknownType(to.to_string()))?;
        let bucket_i = self
            .map
            .get(opt.bucket_resource().name())
            .ok_or_else(|| CibouletteError::UnknownType(opt.bucket_resource().name().clone()))?;
        Ok(CibouletteManyToManyNodeIndexes {
            from: *from_i,
            to: *to_i,
            bucket: *bucket_i,
        })
    }
}
