use super::*;

/// ## Ciboulette configuration
///
/// For the moment, nothing in that struct is used. In the future,
/// this struct will help configure how ciboulette behave when deserializing
/// request and building responses.
#[derive(Getters, MutGetters, Clone, Debug, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteConfig {
    /// The handler base `URL` to build links
    base_url: String,
}
