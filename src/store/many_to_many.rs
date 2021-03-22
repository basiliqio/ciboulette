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

impl<'a> CibouletteStore<'a> {
    /// Add a relationships (one/many-to-one/many) to the graph
    pub fn add_many_to_many_rel(
        &mut self,
        (from, alias_from): (&str, Option<&str>),
        (to, alias_to): (&str, Option<&str>),
        opt: CibouletteRelationshipManyToManyOption<'a>,
    ) -> Result<(), CibouletteError> {
        let node_indexes = self.get_many_to_many_node_indexes(from, to, &opt)?;
        self.check_bucket_exists(node_indexes.bucket(), from, &opt)?;
        let edge_indexes = self.get_many_to_many_edge_indexes(&node_indexes, opt)?;
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
        let from_key = opt.keys_for_type(&from_type)?.to_string();
        let edge_from_i = self.graph_mut().update_edge(
            indexes.bucket(),
            indexes.from(),
            CibouletteRelationshipOption::OneToMany(CibouletteRelationshipOneToManyOption::new(
                from_type,
                bucket_type.clone(),
                from_key,
            )),
        );
        let to_key = opt.keys_for_type(&to_type)?.to_string();
        let edge_to_i = self.graph_mut().update_edge(
            indexes.bucket(),
            indexes.to(),
            CibouletteRelationshipOption::OneToMany(CibouletteRelationshipOneToManyOption::new(
                to_type,
                bucket_type.clone(),
                to_key,
            )),
        );
        let edge_from_i_direct = self.graph_mut().update_edge(
            indexes.from(),
            indexes.to(),
            CibouletteRelationshipOption::ManyToMany(opt.clone()),
        );
        let edge_to_i_direct = self.graph_mut().update_edge(
            indexes.to(),
            indexes.from(),
            CibouletteRelationshipOption::ManyToMany(opt.clone()),
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
