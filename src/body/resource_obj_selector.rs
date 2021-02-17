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
#[derive(Debug, Clone)]
pub enum CibouletteResourceSelector<'a, T> {
    One(CibouletteResource<'a, T>),
    Many(Vec<CibouletteResource<'a, T>>),
}

impl<'a> TryFrom<CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>>
    for CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<Self, Self::Error> {
        match value {
            CibouletteResourceSelector::One(r) => {
                Ok(CibouletteResourceSelector::One(r.try_into()?))
            }
            CibouletteResourceSelector::Many(rs) => Ok(CibouletteResourceSelector::Many(
                rs.into_iter().map(|x| x.try_into()).collect::<Result<
                    Vec<CibouletteResource<'a, CibouletteResourceIdentifier<'a>>>,
                    CibouletteError,
                >>()?,
            )),
        }
    }
}

impl<'a> From<CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>>
    for CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>
{
    fn from(value: CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>) -> Self {
        match value {
			CibouletteResourceSelector::One(r) => CibouletteResourceSelector::One(r.into()),
			CibouletteResourceSelector::Many(rs) => CibouletteResourceSelector::Many(
				rs.into_iter()
					.map(|x| x.into())
					.collect::<Vec<CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>>>(
					),
			),
		}
    }
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
        bag: &'a CibouletteStore,
    ) -> Result<
        CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive>,
        CibouletteError,
    > {
        match self {
            CibouletteResourceSelectorBuilder::One(element) => {
                Ok(CibouletteResourceSelector::One(element.build(bag)?))
            }
            CibouletteResourceSelectorBuilder::Many(elements) => {
                let mut res: Vec<CibouletteResource<CibouletteResourceIdentifierPermissive>> =
                    Vec::with_capacity(elements.len());

                for el in elements.into_iter() {
                    res.push(el.build(bag)?);
                }
                Ok(CibouletteResourceSelector::Many(res))
            }
        }
    }
}

impl<'a, T> CibouletteResourceSelector<'a, T> {
    pub fn check_member_name(&self) -> Result<(), CibouletteError> {
        match self {
            CibouletteResourceSelector::One(element) => element.check_member_name(),
            CibouletteResourceSelector::Many(elements) => {
                for element in elements.iter() {
                    element.check_member_name()?;
                }
                Ok(())
            }
        }
    }
}
