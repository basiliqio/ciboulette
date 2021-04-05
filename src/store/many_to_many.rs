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
    from: petgraph::graph::EdgeIndex<u16>,
    from_direct: petgraph::graph::EdgeIndex<u16>,
    to: petgraph::graph::EdgeIndex<u16>,
    to_direct: petgraph::graph::EdgeIndex<u16>,
}

impl<'a> CibouletteStoreBuilder<'a> {
    /// Add a relationships (one/many-to-one/many) to the graph
    pub fn add_many_to_many_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOptionBuilder<'a>,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let edge_indexes = self.get_many_to_many_edge_indexes(&node_indexes, &opt)?;
        self.add_many_to_many_rel_routine(
            (from, node_indexes.from()),
            (to, alias_to),
            &opt.bucket_resource(),
            &edge_indexes,
            edge_indexes.to_direct(),
        )?;
        self.add_many_to_many_rel_routine(
            (to, node_indexes.to()),
            (from, alias_from),
            &opt.bucket_resource(),
            &edge_indexes,
            edge_indexes.from_direct(),
        )?;
        Ok(())
    }

    fn add_many_to_many_rel_routine(
        &mut self,
        (orig, orig_i): (&str, petgraph::graph::NodeIndex<u16>),
        (dest, alias_dest): (&str, Option<&str>),
        bucket: &CibouletteResourceType<'a>,
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

    fn get_many_to_many_edge_indexes(
        &mut self,
        indexes: &CibouletteManyToManyNodeIndexes,
        opt: &CibouletteRelationshipManyToManyOptionBuilder<'a>,
    ) -> Result<CibouletteManyToManyEdgeIndexes, CibouletteError> {
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
        let edge_from_i_direct = self.graph_mut().add_edge(
            indexes.from(),
            indexes.to(),
            CibouletteRelationshipOptionBuilder::ManyToMany(opt.clone()),
        );
        let edge_to_i_direct = self.graph_mut().add_edge(
            indexes.to(),
            indexes.from(),
            CibouletteRelationshipOptionBuilder::ManyToMany(opt.clone()),
        );
        let from_key = opt.keys_for_type(&from_type)?.to_string();
        let edge_from_i = self.graph_mut().add_edge(
            indexes.bucket(),
            indexes.from(),
            CibouletteRelationshipOptionBuilder::OneToMany(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    from_type,
                    bucket_type.clone(),
                    from_key,
                    false,
                    edge_from_i_direct,
                ),
            ),
        );
        let to_key = opt.keys_for_type(&to_type)?.to_string();
        let edge_to_i = self.graph_mut().add_edge(
            indexes.bucket(),
            indexes.to(),
            CibouletteRelationshipOptionBuilder::OneToMany(
                CibouletteRelationshipOneToManyOptionBuilder::new_from_many_to_many(
                    to_type,
                    bucket_type.clone(),
                    to_key,
                    false,
                    edge_to_i_direct,
                ),
            ),
        );
        Ok(CibouletteManyToManyEdgeIndexes {
            from: edge_from_i,
            to: edge_to_i,
            from_direct: edge_from_i_direct,
            to_direct: edge_to_i_direct,
        })
    }

    fn check_bucket_exists(
        &mut self,
        bucket_i: petgraph::graph::NodeIndex<u16>,
        from: &str,
        opt: &CibouletteRelationshipManyToManyOptionBuilder<'a>,
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
