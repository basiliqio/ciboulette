use super::*;
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
    /// From the source type to the bucket type
    from: petgraph::graph::EdgeIndex<u16>,
    /// From the source type to the dest type
    from_direct: petgraph::graph::EdgeIndex<u16>,
    /// From the dest type to the bucket type
    to: petgraph::graph::EdgeIndex<u16>,
    /// From the dest type to the source type
    to_direct: petgraph::graph::EdgeIndex<u16>,
}

macro_rules! cancel_rel_edge_on_failure {
    ($self:ident, $edge_indexes:ident, $fnc:expr) => {
        if let Err(x) = $fnc {
            $self.cancel_rel_edges(&$edge_indexes);
            return Err(x);
        }
    };
}

impl CibouletteStoreBuilder {
    /// Add a relationships (one/many-to-one/many) to the graph, with the reverse relationship
    pub fn add_many_to_many_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let edge_indexes = self.get_many_to_many_edge_indexes(&node_indexes, &opt)?;
        // Add relationship from `from` to `to`
        cancel_rel_edge_on_failure!(
            self,
            edge_indexes,
            self.add_many_to_many_rel_routine(
                (from, node_indexes.from()),
                (to, alias_to),
                &opt.bucket_resource(),
                edge_indexes.to_direct(),
            )
        );
        // Add relationship from `to` to `from`
        cancel_rel_edge_on_failure!(
            self,
            edge_indexes,
            self.add_many_to_many_rel_routine(
                (to, node_indexes.to()),
                (from, alias_from),
                &opt.bucket_resource(),
                edge_indexes.from_direct(),
            )
        );
        let (edge_index, alias) = match opt.keys()[0].0.name() == from {
            true => (edge_indexes.to(), alias_from),
            false => (edge_indexes.from(), alias_to),
        };
        cancel_rel_edge_on_failure!(
            self,
            edge_indexes,
            self.add_one_to_many_rel(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    opt.keys()[0].0.clone(),
                    opt.keys()[1].1.clone(),
                    opt.bucket_resource().clone(),
                    opt.keys()[0].1.clone(),
                    false,
                    edge_index
                ),
                alias.map(ArcStr::from),
                None,
            )
        );
        let (edge_index, alias) = match opt.keys()[1].0.name() == from {
            true => (edge_indexes.to(), alias_from),
            false => (edge_indexes.from(), alias_to),
        };
        cancel_rel_edge_on_failure!(
            self,
            edge_indexes,
            self.add_one_to_many_rel(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    opt.keys()[1].0.clone(),
                    opt.keys()[0].1.clone(),
                    opt.bucket_resource().clone(),
                    opt.keys()[1].1.clone(),
                    false,
                    edge_index
                ),
                alias.map(ArcStr::from),
                None,
            )
        );
        Ok(())
    }

    /// Add a relationships (one/many-to-one/many) to the graph, without insert OneToMany/ManyToOne
    pub fn add_many_to_many_rel_no_reverse_direct_only(
        &mut self,
        from: &str,
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let (from_type, bucket_type, to_type) = self.extract_many_to_many_types(&node_indexes)?;
        let (edge_to_direct, _) = self.get_many_to_many_edge_indexes_to(
            &bucket_type,
            &from_type,
            to_type,
            &node_indexes,
            &opt,
        )?;
        self.add_many_to_many_rel_routine(
            (from, node_indexes.from()),
            (to, alias_to),
            &opt.bucket_resource(),
            edge_to_direct,
        )?;
        Ok(())
    }

    /// Add a relationships M2M to the graph, but only from `from` to `to` and not the reverse
    pub fn add_many_to_many_rel_no_reverse(
        &mut self,
        from: &str,
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let (from_type, bucket_type, to_type) = self.extract_many_to_many_types(&node_indexes)?;
        let many_resource_key = opt.keys_for_type(&from_type)?;
        let one_resource_key = opt.keys_for_type(&to_type)?;
        let (edge_to_direct, edge_to) = self.get_many_to_many_edge_indexes_to(
            &bucket_type,
            &from_type,
            to_type,
            &node_indexes,
            &opt,
        )?;
        self.add_one_to_many_rel(
            CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                from_type,
                one_resource_key,
                opt.bucket_resource().clone(),
                many_resource_key,
                false,
                edge_to,
            ),
            alias_to.map(ArcStr::from),
            None,
        )?;
        self.add_many_to_many_rel_routine(
            (from, node_indexes.from()),
            (to, alias_to),
            &opt.bucket_resource(),
            edge_to_direct,
        )?;
        Ok(())
    }

    /// Add a relationship M2M
    fn add_many_to_many_rel_routine(
        &mut self,
        (orig, orig_i): (&str, petgraph::graph::NodeIndex<u16>),
        (dest, alias_dest): (&str, Option<&str>),
        bucket: &CibouletteResourceType,
        rel_to_insert: petgraph::graph::EdgeIndex<u16>,
    ) -> Result<(), CibouletteError> {
        let type_ = self
            .graph
            .node_weight_mut(orig_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(orig.to_string()))?;
        let alias = alias_dest.unwrap_or(dest);
        if type_.relationships().contains_key(alias) {
            // Check if relationship exists
            return Err(CibouletteError::UniqRelationship(
                orig.to_string(),
                alias.to_string(),
            ));
        }
        let alias_arc = ArcStr::from(alias);
        let bucket_arc = ArcStr::from(bucket.name());
        type_
            .relationships_mut()
            .insert(alias_arc.clone(), rel_to_insert);
        type_
            .relationships_type_to_alias_mut()
            .insert(ArcStr::from(dest), alias_arc);
        type_
            .relationships_type_to_alias_mut()
            .insert(bucket_arc.clone(), bucket_arc);
        Ok(())
    }

    /// Cancel relationships edges
    fn cancel_rel_edges(&mut self, edge_indexes: &CibouletteManyToManyEdgeIndexes) {
        self.graph.remove_edge(edge_indexes.from());
        self.graph.remove_edge(edge_indexes.to());
        self.graph.remove_edge(edge_indexes.from_direct());
        self.graph.remove_edge(edge_indexes.to_direct());
    }

    /// Create the graph edges for M2M relationships
    fn get_many_to_many_edge_indexes(
        &mut self,
        indexes: &CibouletteManyToManyNodeIndexes,
        opt: &CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<CibouletteManyToManyEdgeIndexes, CibouletteError> {
        let (from_type, bucket_type, to_type) = self.extract_many_to_many_types(indexes)?;
        let (edge_to_direct, edge_to) = self.get_many_to_many_edge_indexes_to(
            &bucket_type,
            &from_type,
            to_type.clone(),
            indexes,
            opt,
        )?;
        let (edge_from_direct, edge_from) = self.get_many_to_many_edge_indexes_from(
            &bucket_type,
            to_type,
            &from_type,
            indexes,
            opt,
        )?;
        Ok(CibouletteManyToManyEdgeIndexes {
            from: edge_from,
            to: edge_to,
            from_direct: edge_from_direct,
            to_direct: edge_to_direct,
        })
    }

    /// Create the edges for a new M2M relationships for destination type
    fn get_many_to_many_edge_indexes_to(
        &mut self,
        bucket_type: &CibouletteResourceType,
        from_type: &CibouletteResourceType,
        to_type: CibouletteResourceType,
        indexes: &CibouletteManyToManyNodeIndexes,
        opt: &CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<
        (
            petgraph::graph::EdgeIndex<u16>,
            petgraph::graph::EdgeIndex<u16>,
        ),
        CibouletteError,
    > {
        let edge_to_i_direct = self.graph_mut().add_edge(
            indexes.to(),
            indexes.from(),
            CibouletteRelationshipOptionBuilder::ManyToMany(opt.clone()),
        );
        let from_key = opt.keys_for_type(&from_type)?;
        let to_key = opt.keys_for_type(&to_type)?;
        let edge_to_i = self.graph_mut().add_edge(
            indexes.bucket(),
            indexes.to(),
            CibouletteRelationshipOptionBuilder::OneToMany(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    to_type,
                    from_key,
                    bucket_type.clone(),
                    to_key,
                    false,
                    edge_to_i_direct,
                ),
            ),
        );
        Ok((edge_to_i_direct, edge_to_i))
    }

    /// Create the edges for a new M2M relationships for source type
    fn get_many_to_many_edge_indexes_from(
        &mut self,
        bucket_type: &CibouletteResourceType,
        from_type: CibouletteResourceType,
        to_type: &CibouletteResourceType,
        indexes: &CibouletteManyToManyNodeIndexes,
        opt: &CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<
        (
            petgraph::graph::EdgeIndex<u16>,
            petgraph::graph::EdgeIndex<u16>,
        ),
        CibouletteError,
    > {
        let edge_from_i_direct = self.graph_mut().add_edge(
            indexes.from(),
            indexes.to(),
            CibouletteRelationshipOptionBuilder::ManyToMany(opt.clone()),
        );
        let from_key = opt.keys_for_type(&from_type)?;
        let to_key = opt.keys_for_type(&to_type)?;
        let edge_from_i = self.graph_mut().add_edge(
            indexes.bucket(),
            indexes.from(),
            CibouletteRelationshipOptionBuilder::OneToMany(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    from_type,
                    to_key,
                    bucket_type.clone(),
                    from_key,
                    false,
                    edge_from_i_direct,
                ),
            ),
        );
        Ok((edge_from_i_direct, edge_from_i))
    }

    /// Get the type from graph node index when building a M2M relationships.
    fn extract_many_to_many_types(
        &mut self,
        indexes: &CibouletteManyToManyNodeIndexes,
    ) -> Result<
        (
            CibouletteResourceType,
            CibouletteResourceType,
            CibouletteResourceType,
        ),
        CibouletteError,
    > {
        let (from_type, to_type, bucket_type) = {
            let from_type = self.graph().node_weight(indexes.from()).ok_or_else(|| {
                CibouletteError::TypeNotInGraph(format!("<index {}>", indexes.from().index()))
            })?;
            let to_type = self.graph().node_weight(indexes.to()).ok_or_else(|| {
                CibouletteError::TypeNotInGraph(format!("<index {}>", indexes.to().index()))
            })?;
            let bucket_type = self.graph().node_weight(indexes.bucket()).ok_or_else(|| {
                CibouletteError::TypeNotInGraph(format!("<index {}>", indexes.bucket().index()))
            })?;
            (from_type.clone(), to_type.clone(), bucket_type.clone())
        };
        Ok((from_type, bucket_type, to_type))
    }

    /// Check that the bucket used in a M2M relationship exists
    fn check_bucket_exists(
        &mut self,
        bucket_i: petgraph::graph::NodeIndex<u16>,
        from: &str,
        opt: &CibouletteRelationshipManyToManyOptionBuilder,
    ) -> Result<(), CibouletteError> {
        let type_fetched = self.graph.node_weight(bucket_i);
        match type_fetched {
            None => return Err(CibouletteError::TypeNotInGraph(from.to_string())), // If it doens't, its an error
            Some(x) if x != opt.bucket_resource() => {
                return Err(CibouletteError::TypeNotInGraph(from.to_string()));
                // If it exists but types aren't equals, it's also an error
            }
            Some(_) => (), // TODO maybe make more checks
        };
        Ok(())
    }

    /// Create the node indexes for the specified type when creating a M2M relationship
    fn get_many_to_many_node_indexes(
        &mut self,
        from: &str,
        to: &str,
        opt: &CibouletteRelationshipManyToManyOptionBuilder,
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
            .get(opt.bucket_resource().name().as_str())
            .ok_or_else(|| {
                CibouletteError::UnknownType(opt.bucket_resource().name().to_string())
            })?;
        Ok(CibouletteManyToManyNodeIndexes {
            from: *from_i,
            to: *to_i,
            bucket: *bucket_i,
        })
    }
}
