use super::*;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserializer,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CibouletteSelectorVisitor<T> {
    pub _marker: std::marker::PhantomData<Option<T>>,
}

impl<'de, T> Default for CibouletteSelectorVisitor<T> {
    fn default() -> Self {
        CibouletteSelectorVisitor {
            _marker: std::marker::PhantomData::default(),
        }
    }
}

/// Builder for the [CibouletteSelector](CibouletteSelector)
#[derive(Debug, Clone)]
pub(crate) enum CibouletteSelectorBuilder<'de, T> {
    Single(MessyJsonValueRaw<'de>),
    Multi(Vec<T>),
}

impl<'de, T> Deserialize<'de> for CibouletteSelector<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match deserializer.deserialize_any(CibouletteSelectorVisitor::default())? {
            CibouletteSelectorBuilder::Multi(result) => Ok(CibouletteSelector::Multi(result)),
            CibouletteSelectorBuilder::Single(value) => Ok(CibouletteSelector::Single(
                T::deserialize(value).map_err(|err| serde::de::Error::custom(err.to_string()))?,
            )),
        }
    }
}

impl<'de, T> Visitor<'de> for CibouletteSelectorVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = CibouletteSelectorBuilder<'de, T>;

    #[inline]
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a value or an array of values")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut res: Vec<T> = Vec::with_capacity(seq.size_hint().unwrap_or_default());

        while let Some(next) = seq.next_element()? {
            res.push(next);
        }
        Ok(CibouletteSelectorBuilder::Multi(res))
    }

    #[inline]
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(CibouletteSelectorBuilder::Single(
            MessyJsonValueRawVisitor::visit_map(MessyJsonValueRawVisitor::default(), map)?,
        ))
    }
    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(MessyJsonValueRaw::Bool(
            v,
        )))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(
            MessyJsonValueRawVisitor::visit_i64(MessyJsonValueRawVisitor::default(), v)?,
        ))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(
            MessyJsonValueRawVisitor::visit_u64(MessyJsonValueRawVisitor::default(), v)?,
        ))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(
            MessyJsonValueRawVisitor::visit_f64(MessyJsonValueRawVisitor::default(), v)?,
        ))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(
            MessyJsonValueRawVisitor::visit_str(MessyJsonValueRawVisitor::default(), v)?,
        ))
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(MessyJsonValueRaw::Null))
    }
}
