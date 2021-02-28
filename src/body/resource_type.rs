use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType<'a> {
    relationships: BTreeMap<String, petgraph::graph::EdgeIndex<u16>>,
    relationships_type_to_alias: BTreeMap<String, String>,
    schema: MessyJsonObject<'a>,
    name: String,
}

impl<'a> CibouletteResourceType<'a> {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, schema: MessyJsonObject<'a>) -> Self {
        CibouletteResourceType {
            relationships: BTreeMap::new(),
            relationships_type_to_alias: BTreeMap::new(),
            schema,
            name,
        }
    }

    /// Get a the alias of a type related to this type
    pub fn get_alias(&self, name: &str) -> Result<&String, CibouletteError> {
        self.relationships_type_to_alias().get(name).ok_or_else(|| {
            CibouletteError::MissingAliasTranslation(self.name().to_string(), name.to_string())
        })
    }

    pub fn has_fields(&self, fields: &[&str]) -> Result<Option<String>, CibouletteError> {
        Ok(fields.iter().find_map(|k| match self.schema.has_field(*k) {
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
