use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType {
    relationships: BTreeSet<String>,
    schema: MessyJson,
}

impl CibouletteResourceType {
    /// Create a new type from a schema and a list of relationships
    pub fn new(schema: MessyJson, relationships: Vec<String>) -> Self {
        CibouletteResourceType {
            relationships: relationships.into_iter().collect(),
            schema,
        }
    }
}
