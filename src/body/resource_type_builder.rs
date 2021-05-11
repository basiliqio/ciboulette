use lazy_static::__Deref;

use super::*;

/// ## Describe a `json:api` type attribute schema and list its relationships
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceTypeBuilder {
    relationships: BTreeMap<ArcStr, petgraph::graph::EdgeIndex<u16>>,
    relationships_type_to_alias: BTreeMap<ArcStr, ArcStr>,
    schema: MessyJsonObject,
    ids: CibouletteIdTypeSelector,
    name: ArcStr,
}

impl CibouletteResourceTypeBuilder {
    /// Create a new type from a schema and a list of relationships
    pub fn new(name: String, id_type: CibouletteIdTypeSelector, schema: MessyJsonObject) -> Self {
        CibouletteResourceTypeBuilder {
            relationships: BTreeMap::new(),
            relationships_type_to_alias: BTreeMap::new(),
            schema,
            ids: id_type,
            name: ArcStr::from(name),
        }
    }

    /// Check the the name of every field in an object.
    ///
    /// Return `Some(field)` if a field doesn't respect the member name's checks.
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

    /// Check the the name of every field in an document.
    ///
    /// Return `Some(field)` if a field doesn't respect the member name's checks.
    fn check_member_name(val: &MessyJson) -> Option<String> {
        match val.deref() {
            MessyJsonInner::Obj(map) => Self::check_member_name_obj(map),
            MessyJsonInner::Array(arr) => Self::check_member_name(arr.items()),
            _ => None,
        }
    }

    /// Build the resource type, checking once the member names
    pub fn build(self) -> Result<CibouletteResourceType, CibouletteError> {
        if let Some(x) = Self::check_member_name_obj(self.schema()) {
            return Err(CibouletteError::InvalidMemberName(x));
        }
        Ok(CibouletteResourceType::new(
            self.name,
            self.ids,
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
