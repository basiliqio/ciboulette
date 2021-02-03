use super::*;

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType<'a> {
    relationships: HashMap<Cow<'a, str>, CibouletteResourceType<'a>>,
    schema: MessyJson,
    schema_sparse: MessyJson,
}
