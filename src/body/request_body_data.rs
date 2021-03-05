use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum CibouletteBodyDataBuilder<'a> {
    Object(CibouletteResourceSelectorBuilder<'a>),
    Null(bool),
}

impl<'a> Default for CibouletteBodyDataBuilder<'a> {
    fn default() -> Self {
        CibouletteBodyDataBuilder::Null(false)
    }
}

#[derive(Debug)]
pub enum CibouletteBodyData<'a> {
    Object(CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>),
    Null(bool),
}

impl<'a> Default for CibouletteBodyData<'a> {
    fn default() -> Self {
        CibouletteBodyData::Null(false)
    }
}

impl<'a> CibouletteBodyDataBuilder<'a> {
    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
        intention: &CibouletteIntention,
    ) -> Result<CibouletteBodyData<'a>, CibouletteError> {
        match self {
            CibouletteBodyDataBuilder::Object(x) => {
                Ok(CibouletteBodyData::Object(x.build(bag, intention)?))
            }
            CibouletteBodyDataBuilder::Null(x) => Ok(CibouletteBodyData::Null(x)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CibouletteRequestBodyDataBuilderVisitor;

impl<'de> Deserialize<'de> for CibouletteBodyDataBuilder<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(CibouletteRequestBodyDataBuilderVisitor::default())
    }
}

impl<'de> Visitor<'de> for CibouletteRequestBodyDataBuilderVisitor {
    type Value = CibouletteBodyDataBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(
            formatter,
            "a data object, an identifier, a list of identifier or null",
        )
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CibouletteBodyDataBuilder::Object(
            CibouletteResourceSelectorBuilder::deserialize(deserializer)?,
        ))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CibouletteBodyDataBuilder::Null(true))
    }
}
