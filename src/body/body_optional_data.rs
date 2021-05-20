use super::*;
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt::Formatter;

/// ## Wrapper for optional data
///
/// Providing a nuance between explicitely null-ed values and absent ones.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CibouletteOptionalData<T> {
    Object(T),
    #[serde(serialize_with = "serialize_null_for_optional_data")]
    Null(bool),
}

impl<T> CibouletteOptionalData<T>
where
    T: std::ops::Deref,
    <T as std::ops::Deref>::Target: Sized,
{
    pub fn inner_deref(&self) -> CibouletteOptionalData<&<T as std::ops::Deref>::Target> {
        match self {
            CibouletteOptionalData::Object(x) => CibouletteOptionalData::Object(&**x),
            CibouletteOptionalData::Null(x) => CibouletteOptionalData::Null(*x),
        }
    }
}

impl<T> Default for CibouletteOptionalData<T> {
    fn default() -> Self {
        CibouletteOptionalData::Null(false)
    }
}

fn serialize_null_for_optional_data<S>(_val: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_none()
}

impl<T> CibouletteOptionalData<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, CibouletteOptionalData::Null(false))
    }
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
