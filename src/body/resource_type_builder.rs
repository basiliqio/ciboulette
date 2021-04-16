use lazy_static::__Deref;

use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceTypeBuilder {
    relationships: BTreeMap<ArcStr, petgraph::graph::EdgeIndex<u16>>,
    relationships_type_to_alias: BTreeMap<ArcStr, ArcStr>,
    schema: MessyJsonObject,
    id_type: CibouletteIdType,
    name: ArcStr,
}

impl CibouletteResourceTypeBuilder {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, id_type: CibouletteIdType, schema: MessyJsonObject) -> Self {
        CibouletteResourceTypeBuilder {
            relationships: BTreeMap::new(),
            relationships_type_to_alias: BTreeMap::new(),
            schema,
            id_type,
            name: ArcStr::from(name),
        }
    }
    fn check_member_name_obj(val: &MessyJsonObject) -> Option<String> {
        for (k, v) in val.properties().iter() {
            if !crate::member_name::check_member_name(&*k) {
                return Some(k.to_string());
            }
            if let Some(x) = Self::check_member_name(v) {
                return Some(x);
            }
        }
        None
    }
    fn check_member_name(val: &MessyJson) -> Option<String> {
        match val.deref() {
            MessyJsonInner::Obj(map) => Self::check_member_name_obj(map),
            MessyJsonInner::Array(arr) => Self::check_member_name(arr.items()),
            _ => None,
        }
    }
    pub fn build(self) -> Result<CibouletteResourceType, CibouletteError> {
        if let Some(x) = Self::check_member_name_obj(self.schema()) {
            return Err(CibouletteError::InvalidMemberName(x));
        }
        Ok(CibouletteResourceType::new(
            self.name,
            self.id_type,
            self.schema,
        ))
    }
}

impl Ord for CibouletteResourceTypeBuilder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for CibouletteResourceTypeBuilder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PartialEq for CibouletteResourceTypeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CibouletteResourceTypeBuilder {}
