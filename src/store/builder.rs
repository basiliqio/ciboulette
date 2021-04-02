use super::*;

/// ## Map of accepted resource types
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct CibouletteStoreBuilder<'a> {
    pub(crate) graph: petgraph::graph::Graph<
        CibouletteResourceType<'a>,
        CibouletteRelationshipOptionBuilder<'a>,
        petgraph::Directed,
        u16,
    >,
    pub(crate) map: BTreeMap<String, petgraph::graph::NodeIndex<u16>>,
    pub(crate) config: CibouletteConfig,
}

impl<'a> Default for CibouletteStoreBuilder<'a> {
    #[inline]
    fn default() -> Self {
        CibouletteStoreBuilder {
            graph: petgraph::graph::Graph::with_capacity(0, 0),
            map: BTreeMap::new(),
            config: CibouletteConfig::default(),
        }
    }
}

impl<'a> CibouletteStoreBuilder<'a> {
    /// Create a new bag
    #[inline]
    pub fn new(config: CibouletteConfig) -> Self {
        CibouletteStoreBuilder {
            config,
            graph: petgraph::graph::Graph::default(),
            map: BTreeMap::default(),
        }
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
    ) -> Result<
        (
            &'a CibouletteResourceType,
            &'a CibouletteRelationshipOptionBuilder,
        ),
        CibouletteError,
    > {
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
        name: &str,
        id_type: CibouletteIdType,
        schema: MessyJsonObject<'a>,
    ) -> Result<(), CibouletteError> {
        let name = name.to_string();
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

    pub fn build(self) -> Result<CibouletteStore<'a>, CibouletteError> {
        let mut tmp_graph: petgraph::graph::Graph<
            Arc<CibouletteResourceType<'a>>,
            CibouletteRelationshipOption<'a>,
            petgraph::Directed,
            u16,
        > = petgraph::graph::Graph::with_capacity(
            self.graph().node_count(),
            self.graph.edge_count(),
        );

        for node in self.graph().raw_nodes() {
            tmp_graph.add_node(Arc::new(node.weight.clone()));
        }
        for edge in self.graph().raw_edges() {
            tmp_graph.add_edge(
                edge.source(),
                edge.target(),
                edge.weight.build(&self, &tmp_graph)?,
            );
        }
        Ok(CibouletteStore {
            config: self.config,
            map: self.map,
            graph: tmp_graph,
        })
    }
}
