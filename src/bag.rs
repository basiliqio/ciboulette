use super::*;
use serde::de::{DeserializeSeed, Deserializer};

#[derive(Clone, Debug, Default)]
pub struct CibouletteBag<'a>(Arc<HashMap<Cow<'a, str>, CibouletteResourceType<'a>>>);

impl<'a> CibouletteBag<'a> {
    pub fn map(&self) -> &Arc<HashMap<Cow<'a, str>, CibouletteResourceType<'a>>> {
        &self.0
    }
}
