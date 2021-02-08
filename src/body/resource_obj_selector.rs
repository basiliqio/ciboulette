use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug)]
pub enum CibouletteResourceSelectorBuilder<'a> {
    One(CibouletteResourceBuilder<'a>),
    Many(Vec<CibouletteResourceBuilder<'a>>),
}

/// ## A selector between a single or multiple `json:api` [resource](https://jsonapi.org/format/#document-resource-objects) objects
#[derive(Debug)]
pub enum CibouletteResourceSelector<'a> {
    One(CibouletteResource<'a>),
    Many(Vec<CibouletteResource<'a>>),
}

#[derive(Clone, Debug)]
struct CibouletteResourceSelectorBuilderVisitor;

impl<'de> serde::de::Visitor<'de> for CibouletteResourceSelectorBuilderVisitor {
    type Value = CibouletteResourceSelectorBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResourceSelector")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res: Vec<CibouletteResourceBuilder<'de>> =
            Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(v) = seq.next_element_seed(CibouletteResourceBuilderVisitor)? {
            res.push(v);
        }
        Ok(CibouletteResourceSelectorBuilder::Many(res))
    }

    #[inline]
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(CibouletteResourceSelectorBuilder::One(
            <CibouletteResourceBuilderVisitor as Visitor>::visit_map(
                CibouletteResourceBuilderVisitor,
                map,
            )?,
        ))
    }
}

impl<'de> Deserialize<'de> for CibouletteResourceSelectorBuilder<'de> {
    fn deserialize<D>(deserializer: D) -> Result<CibouletteResourceSelectorBuilder<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CibouletteResourceSelectorBuilderVisitor)
    }
}

impl<'a> CibouletteResourceSelectorBuilder<'a> {
    /// Build the resource selector from the builder
    pub fn build(
        self,
        bag: &'a CibouletteBag,
    ) -> Result<CibouletteResourceSelector<'a>, CibouletteError> {
        match self {
            CibouletteResourceSelectorBuilder::One(element) => {
                Ok(CibouletteResourceSelector::One(element.build(bag)?))
            }
            CibouletteResourceSelectorBuilder::Many(elements) => {
                let mut res: Vec<CibouletteResource> = Vec::with_capacity(elements.len());

                for el in elements.into_iter() {
                    res.push(el.build(bag)?);
                }
                Ok(CibouletteResourceSelector::Many(res))
            }
        }
    }
}
