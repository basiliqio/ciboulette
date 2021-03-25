use super::*;

#[derive(Getters, MutGetters, Clone, Debug, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteConfig {
    base_url: String,
}
