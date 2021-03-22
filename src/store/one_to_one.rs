use super::*;

impl<'a> CibouletteStore<'a> {
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
}
