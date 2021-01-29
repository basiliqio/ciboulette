use super::*;

#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteConfig {
    base_url: String,
    types: HashMap<String, String>, //TODO types
}
