use super::*;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserializer,
};

#[derive(Debug, Clone, Copy)]
struct CibouletteSelectorVisitor<'de, T> {
    pub _marker: std::marker::PhantomData<Option<&'de T>>,
}

impl<'de, T> Default for CibouletteSelectorVisitor<'de, T> {
    fn default() -> Self {
        CibouletteSelectorVisitor {
            _marker: std::marker::PhantomData::default(),
        }
    }
}

/// Builder for the [CibouletteSelector](CibouletteSelector)
#[derive(Debug, Clone)]
enum CibouletteSelectorBuilder<T> {
    Single(Value),
    Multi(Vec<T>),
}

impl<'de, T> Deserialize<'de> for CibouletteSelector<T>
where
    T: 'de + Deserialize<'de>,
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

impl<'de, T> Visitor<'de> for CibouletteSelectorVisitor<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = CibouletteSelectorBuilder<T>;

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
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::from(v)))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::Null))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CibouletteSelectorBuilder::Single(Value::Null))
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut res = serde_json::Map::with_capacity(map.size_hint().unwrap_or_default());

        while let Some(el) = map.next_entry()? {
            res.insert(el.0, el.1);
        }
        Ok(CibouletteSelectorBuilder::Single(Value::from(res)))
    }
}
