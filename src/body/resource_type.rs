use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType {
    relationships: BTreeMap<String, petgraph::graph::EdgeIndex<u16>>,
    relationships_type_to_alias: BTreeMap<String, String>,
    schema: MessyJson,
    name: String,
}

impl CibouletteResourceType {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, schema: MessyJson) -> Self {
        CibouletteResourceType {
            relationships: BTreeMap::new(),
            relationships_type_to_alias: BTreeMap::new(),
            schema,
            name,
        }
    }

    /// Get a the alias of a type related to this type
    pub fn get_alias<'a>(&'a self, name: &str) -> Result<&String, CibouletteError> {
        self.relationships_type_to_alias().get(name).ok_or_else(|| {
            CibouletteError::MissingAliasTranslation(self.name().to_string(), name.to_string())
        })
    }
}

impl Ord for CibouletteResourceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for CibouletteResourceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PartialEq for CibouletteResourceType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CibouletteResourceType {}
