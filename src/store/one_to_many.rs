use super::*;

impl CibouletteStoreBuilder {
    /// Add a relationships (one-to-many) to the graph
    pub fn add_one_to_many_rel(
        &mut self,
        opt: CibouletteRelationshipOneToManyOptionBuilder,
        alias_one_resource: Option<ArcStr>,
        alias_many_resource: Option<ArcStr>,
    ) -> Result<(), CibouletteError> {
        let (from_i, to_i) = self.get_one_to_many_node_indexes(&opt)?;
        let (from_rel_i, to_rel_i) = self.get_one_to_many_edge_indexes(&from_i, &to_i, &opt);
        // From source to dest
        self.add_one_to_many_rel_routine(
            opt.one_resource(),
            opt.many_resource(),
            from_i,
            from_rel_i,
            alias_many_resource,
            to_rel_i,
        )?;
        // From dest to source
        self.add_one_to_many_rel_routine(
            opt.many_resource(),
            opt.one_resource(),
            to_i,
            to_rel_i,
            alias_one_resource.or_else(|| Some(opt.many_resource_key().clone())),
            from_rel_i,
        )?;
        Ok(())
    }

    /// Add a relationships (one-to-many) to the graph, without the reverse relationship
    pub fn add_one_to_many_rel_no_reverse(
        &mut self,
        opt: CibouletteRelationshipOneToManyOptionBuilder,
        alias_many_resource: Option<ArcStr>,
    ) -> Result<(), CibouletteError> {
        let (from_i, to_i) = self.get_one_to_many_node_indexes(&opt)?;
        let (from_rel_i, to_rel_i) = self.get_one_to_many_edge_indexes(&from_i, &to_i, &opt);
        // From source to dest
        self.add_one_to_many_rel_routine(
            opt.one_resource(),
            opt.many_resource(),
            from_i,
            from_rel_i,
            alias_many_resource,
            to_rel_i,
        )?;
        Ok(())
    }

    /// Add a relationships (one-to-many) to the graph, without the reverse relationship
    pub fn add_many_to_one_rel_no_reverse(
        &mut self,
        opt: CibouletteRelationshipOneToManyOptionBuilder,
        alias_one_resource: Option<ArcStr>,
    ) -> Result<(), CibouletteError> {
        let (from_i, to_i) = self.get_one_to_many_node_indexes(&opt)?;
        let (from_rel_i, to_rel_i) = self.get_one_to_many_edge_indexes(&from_i, &to_i, &opt);
        // From source to dest
        self.add_one_to_many_rel_routine(
            opt.many_resource(),
            opt.one_resource(),
            to_i,
            to_rel_i,
            alias_one_resource.or_else(|| Some(opt.many_resource_key().clone())),
            from_rel_i,
        )?;
        Ok(())
    }

    fn add_one_to_many_rel_routine(
        &mut self,
        orig: &CibouletteResourceType,
        dest: &CibouletteResourceType,
        orig_i: petgraph::graph::NodeIndex<u16>,
        orig_rel_i: petgraph::graph::EdgeIndex<u16>,
        alias_dest: Option<ArcStr>,
        dest_rel_i: petgraph::graph::EdgeIndex<u16>,
    ) -> Result<(), CibouletteError> {
        let type_ = self
            .graph
            .node_weight_mut(orig_i)
            .ok_or_else(|| CibouletteError::TypeNotInGraph(orig.name().to_string()))?;
        let alias = alias_dest.unwrap_or_else(|| dest.name().clone());
        if type_.relationships().contains_key(&alias) {
            // Check if relationship exists
            self.graph.remove_edge(orig_rel_i); // Cancel the created edge
            self.graph.remove_edge(dest_rel_i);
            return Err(CibouletteError::UniqRelationship(
                orig.name().to_string(),
                alias.to_string(),
            ));
        }
        type_.relationships_mut().insert(alias.clone(), orig_rel_i);
        type_
            .relationships_type_to_alias_mut()
            .insert(ArcStr::from(dest.name()), alias);
        Ok(())
    }

    /// Get the edge indexes for a O2M or M2O relationships
    fn get_one_to_many_edge_indexes(
        &mut self,
        from_i: &petgraph::graph::NodeIndex<u16>,
        to_i: &petgraph::graph::NodeIndex<u16>,
        opt: &CibouletteRelationshipOneToManyOptionBuilder,
    ) -> (
        petgraph::graph::EdgeIndex<u16>,
        petgraph::graph::EdgeIndex<u16>,
    ) {
        let edge_from_i = self.graph_mut().add_edge(
            *from_i,
            *to_i,
            CibouletteRelationshipOptionBuilder::OneToMany(opt.clone()),
        );
        let edge_to_i = self.graph_mut().add_edge(
            *to_i,
            *from_i,
            CibouletteRelationshipOptionBuilder::ManyToOne(opt.clone()),
        );
        (edge_from_i, edge_to_i)
    }

    /// Get the node indexes for O2M or M2O relationships
    fn get_one_to_many_node_indexes(
        &mut self,
        opt: &CibouletteRelationshipOneToManyOptionBuilder,
    ) -> Result<
        (
            petgraph::graph::NodeIndex<u16>,
            petgraph::graph::NodeIndex<u16>,
        ),
        CibouletteError,
    > {
        let from_i = self
            .map
            .get(opt.one_resource().name().as_str())
            .ok_or_else(|| CibouletteError::UnknownType(opt.one_resource().name().to_string()))?;
        let to_i = self
            .map
            .get(opt.many_resource().name().as_str())
            .ok_or_else(|| CibouletteError::UnknownType(opt.many_resource().name().to_string()))?;
        Ok((*from_i, *to_i))
    }
}
