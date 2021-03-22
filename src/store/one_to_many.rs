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
    /// Add a relationships (one-to-many) to the graph
    pub fn add_one_to_many_rel(
        &mut self,
        opt: CibouletteRelationshipOneToManyOption<'a>,
        alias_one_table: Option<&str>,
        alias_many_table: Option<&str>,
    ) -> Result<(), CibouletteError> {
        let (from_i, to_i) = self.get_one_to_many_node_indexes(&opt)?;
        let (from_rel_i, to_rel_i) = self.get_one_to_many_edge_indexes(&from_i, &to_i, &opt);
        self.add_one_to_many_rel_routine(
            opt.one_table(),
            opt.many_table(),
            from_i,
            from_rel_i,
            alias_many_table,
            to_rel_i,
        )?;
        self.add_one_to_many_rel_routine(
            opt.many_table(),
            opt.one_table(),
            to_i,
            to_rel_i,
            alias_one_table,
            from_rel_i,
        )?;
        Ok(())
    }

    fn add_one_to_many_rel_routine(
        &mut self,
        orig: &CibouletteResourceType<'a>,
        dest: &CibouletteResourceType<'a>,
        orig_i: petgraph::graph::NodeIndex<u16>,
        orig_rel_i: petgraph::graph::EdgeIndex<u16>,
        alias_dest: Option<&str>,
        dest_rel_i: petgraph::graph::EdgeIndex<u16>,
    ) -> Result<(), CibouletteError> {
        let type_ = self
            .graph
            .node_weight_mut(orig_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(orig.name().to_string()))?;
        let alias = alias_dest.unwrap_or_else(|| dest.name().as_str());
        if type_.relationships().contains_key(alias) {
            // Check if relationship exists
            self.graph.remove_edge(orig_rel_i); // Cancel the created edge
            self.graph.remove_edge(dest_rel_i);
            return Err(CibouletteError::UniqRelationship(
                orig.name().to_string(),
                alias.to_string(),
            ));
        }
        type_
            .relationships_mut()
            .insert(alias.to_string(), orig_rel_i);
        type_
            .relationships_type_to_alias_mut()
            .insert(dest.name().to_string(), alias.to_string());
        Ok(())
    }

    fn get_one_to_many_edge_indexes(
        &mut self,
        from_i: &petgraph::graph::NodeIndex<u16>,
        to_i: &petgraph::graph::NodeIndex<u16>,
        opt: &CibouletteRelationshipOneToManyOption<'a>,
    ) -> (
        petgraph::graph::EdgeIndex<u16>,
        petgraph::graph::EdgeIndex<u16>,
    ) {
        let edge_from_i = self.graph_mut().update_edge(
            *from_i,
            *to_i,
            CibouletteRelationshipOption::OneToMany(opt.clone()),
        );
        let edge_to_i = self.graph_mut().update_edge(
            *to_i,
            *from_i,
            CibouletteRelationshipOption::OneToMany(opt.clone()),
        );
        (edge_from_i, edge_to_i)
    }

    fn get_one_to_many_node_indexes(
        &mut self,
        opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<
        (
            petgraph::graph::NodeIndex<u16>,
            petgraph::graph::NodeIndex<u16>,
        ),
        CibouletteError,
    > {
        let from_i = self
            .map
            .get(opt.one_table().name())
            .ok_or_else(|| CibouletteError::UnknownType(opt.one_table().name().to_string()))?;
        let to_i = self
            .map
            .get(opt.many_table().name())
            .ok_or_else(|| CibouletteError::UnknownType(opt.many_table().name().to_string()))?;
        Ok((*from_i, *to_i))
    }
}