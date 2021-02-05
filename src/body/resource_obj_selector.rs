use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct CibouletteResourceSelectorVisitor<'a>(&'a CibouletteBag<'a>);

impl<'a> CibouletteResourceSelectorVisitor<'a> {
    #[inline]
    pub fn new(bag: &'a CibouletteBag<'a>) -> Self {
        CibouletteResourceSelectorVisitor(bag)
    }
}

impl<'de> serde::de::Visitor<'de> for CibouletteResourceSelectorVisitor<'de> {
    type Value = CibouletteResourceSelector<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResourceSelector")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res: Vec<CibouletteResource<'de>> =
            Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(v) = seq.next_element_seed(CibouletteResourceVisitor::new(self.0))? {
            res.push(v);
        }
        Ok(CibouletteResourceSelector::Many(res))
    }

    #[inline]
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let v = CibouletteResourceVisitor::new(self.0);
        Ok(CibouletteResourceSelector::One(
            <CibouletteResourceVisitor as Visitor>::visit_map(v, map)?,
        ))
    }
    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CibouletteResourceSelector::Null)
    }
}

#[derive(Debug)]
pub enum CibouletteResourceSelector<'a> {
    One(CibouletteResource<'a>),
    Many(Vec<CibouletteResource<'a>>),
    Null,
}

impl<'a> CibouletteResourceSelector<'a> {
    #[inline]
    pub fn deserialize<R>(
        d: &mut serde_json::Deserializer<R>,
        bag: &'a CibouletteBag,
    ) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteResourceSelectorVisitor(bag);

        visitor.deserialize(d)
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteResourceSelectorVisitor<'de> {
    type Value = CibouletteResourceSelector<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CibouletteResourceSelectorVisitor(self.0))
    }
}
