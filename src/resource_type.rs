use super::*;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceType<'a> {
    relationships: HashMap<Cow<'a, str>, &'a CibouletteResourceType<'a>>,
    schema: MessyJson,
}

impl<'a> CibouletteResourceType<'a> {
    pub fn new(schema: MessyJson) -> Self {
        CibouletteResourceType {
            relationships: HashMap::new(),
            schema,
        }
    }
}
