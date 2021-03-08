use super::*;
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize)]
pub enum CibouletteOptionalData<T> {
    Object(T),
    Null(bool),
}

#[derive(Clone, Copy, Debug)]
pub struct CibouletteOptionalDataVisitor<T> {
    marker: Option<std::marker::PhantomData<T>>,
}

impl<'de, T> Deserialize<'de> for CibouletteOptionalData<T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(CibouletteOptionalDataVisitor { marker: None })
    }
}

impl<'de, T> Visitor<'de> for CibouletteOptionalDataVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = CibouletteOptionalData<T>;

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
        Ok(CibouletteOptionalData::Object(Deserialize::deserialize(
            deserializer,
        )?))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CibouletteOptionalData::Null(true))
    }
}
