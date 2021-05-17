use super::*;
use getset::CopyGetters;

/// ## Ciboulette configuration
#[derive(Getters, MutGetters, CopyGetters, Clone, Debug)]
pub struct CibouletteConfig {
    /// The url to prepend every links with
    #[getset(get = "pub", get_mut = "pub")]
    base_url: Option<String>,
    /// True if Ciboulette should generate root level links. Default `true`
    #[getset(get_copy = "pub", get_mut = "pub")]
    gen_root_links: bool,
    /// True if Ciboulette should generate resource level self links. Default `true`
    #[getset(get_copy = "pub", get_mut = "pub")]
    gen_resource_links: bool,
    /// True if Ciboulette should generate relationship level self links. Default `true`
    #[getset(get_copy = "pub", get_mut = "pub")]
    gen_relationship_links: bool,
}

impl Default for CibouletteConfig {
    fn default() -> Self {
        CibouletteConfig {
            gen_resource_links: true,
            gen_root_links: true,
            gen_relationship_links: true,
            base_url: None,
        }
    }
}
