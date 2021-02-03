use super::*;
use serde::de::{DeserializeSeed, Deserializer};

#[derive(Clone, Debug, Default)]
pub struct CibouletteBag<'a>(Arc<HashMap<Cow<'a, str>, CibouletteResourceType<'a>>>);
