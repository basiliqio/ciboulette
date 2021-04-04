use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters, Hash)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType<'a> {
    relationships: BTreeMap<ArcStr, petgraph::graph::EdgeIndex<u16>>,
    relationships_type_to_alias: BTreeMap<ArcStr, ArcStr>,
    schema: MessyJsonObject<'a>,
    id_type: CibouletteIdType,
    name: ArcStr,
}

impl<'a> CibouletteResourceType<'a> {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, id_type: CibouletteIdType, schema: MessyJsonObject<'a>) -> Self {
        CibouletteResourceType {
            relationships: BTreeMap::new(),
            relationships_type_to_alias: BTreeMap::new(),
            schema,
            id_type,
            name: ArcStr::from(name),
        }
    }

    /// Get a the alias of a type related to this type
    pub fn get_alias(&self, name: &str) -> Result<&ArcStr, CibouletteError> {
        self.relationships_type_to_alias().get(name).ok_or_else(|| {
            CibouletteError::MissingAliasTranslation(self.name().to_string(), name.to_string())
        })
    }

    pub fn get_relationship(
        &self,
        store: &'a CibouletteStore<'a>,
        alias: &str,
    ) -> Result<&Arc<CibouletteResourceType<'a>>, CibouletteError> {
        let edge_index = self.relationships().get(alias).ok_or_else(|| {
            CibouletteError::UnknownRelationship(self.name().to_string(), alias.to_string())
        })?;
        let self_index = *store
            .map()
            .get(self.name().as_str())
            .ok_or_else(|| CibouletteError::UnknownType(self.name().to_string()))?;
        let (t1, t2) = store.graph().edge_endpoints(*edge_index).ok_or_else(|| {
            CibouletteError::RelNotInGraph(self.name().to_string(), alias.to_string())
        })?;
        Ok(match t1 == self_index {
            true => store.graph().node_weight(t2).ok_or_else(|| {
                CibouletteError::RelNotInGraph(self.name().to_string(), alias.to_string())
            })?,
            false => store.graph().node_weight(t1).ok_or_else(|| {
                CibouletteError::RelNotInGraph(self.name().to_string(), alias.to_string())
            })?,
        })
    }

    pub fn has_fields<'b, I>(&self, fields: I) -> Result<Option<String>, CibouletteError>
    where
        I: Iterator<Item = &'b str>,
    {
        Ok(fields
            .into_iter()
            .find_map(|k| match self.schema.has_field(k) {
                true => None,
                false => Some(k.to_string()),
            }))
    }
}

impl<'a> Ord for CibouletteResourceType<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl<'a> PartialOrd for CibouletteResourceType<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl<'a> PartialEq for CibouletteResourceType<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for CibouletteResourceType<'a> {}
