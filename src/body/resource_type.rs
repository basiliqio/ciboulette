use super::*;

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType {
    relationships: BTreeSet<String>,
    schema: MessyJson,
}

impl CibouletteResourceType {
    pub fn new(schema: MessyJson, relationships: Vec<String>) -> Self {
        CibouletteResourceType {
            relationships: relationships.into_iter().collect(),
            schema,
        }
    }
}
