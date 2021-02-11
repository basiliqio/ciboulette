use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType {
    relationships: BTreeMap<String, String>,
    schema: MessyJson,
    name: String,
}

impl CibouletteResourceType {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, schema: MessyJson, relationships: Vec<(String, String)>) -> Self {
        CibouletteResourceType {
            relationships: relationships.into_iter().collect(),
            schema,
            name,
        }
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
