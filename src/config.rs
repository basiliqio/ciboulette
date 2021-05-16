use super::*;

/// ## Ciboulette configuration
#[derive(Getters, MutGetters, Clone, Debug, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteConfig {
    /// The url to prepend every links with
    base_url: Option<String>,
}
