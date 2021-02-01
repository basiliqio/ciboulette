use super::*;

#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType<'a> {
    relationships: HashMap<Cow<'a, str>, Arc<CibouletteResourceType<'a>>>, // TODO reg
    schema: CibouletteResourceSchema,
}
