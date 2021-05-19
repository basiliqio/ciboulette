use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug)]
pub enum CibouletteResourceSelectorBuilder<'request> {
    One(CibouletteResourceBuilder<'request>),
    Many(Vec<CibouletteResourceBuilder<'request>>),
}

/// ## A selector between a single or multiple `json:api` [resource](https://jsonapi.org/format/#document-resource-objects) objects
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CibouletteResourceSelector<'request, B, T> {
    One(CibouletteResource<'request, B, T>),
    Many(Vec<CibouletteResource<'request, B, T>>),
}

impl<'request, B>
    TryFrom<
        CibouletteResourceSelector<'request, B, CibouletteResourceIdentifierPermissive<'request>>,
    > for CibouletteResourceSelector<'request, B, CibouletteResourceIdentifier<'request>>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResourceSelector<
            'request,
            B,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<Self, Self::Error> {
        match value {
            CibouletteResourceSelector::One(r) => {
                Ok(CibouletteResourceSelector::One(r.try_into()?))
            }
            CibouletteResourceSelector::Many(rs) => Ok(CibouletteResourceSelector::Many(
                rs.into_iter().map(|x| x.try_into()).collect::<Result<
                    Vec<CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>>,
                    CibouletteError,
                >>()?,
            )),
        }
    }
}

impl<'request, B>
    From<CibouletteResourceSelector<'request, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResourceSelector<'request, B, CibouletteResourceIdentifierPermissive<'request>>
{
    fn from(
        value: CibouletteResourceSelector<'request, B, CibouletteResourceIdentifier<'request>>,
    ) -> Self {
        match value {
            CibouletteResourceSelector::One(r) => CibouletteResourceSelector::One(r.into()),
            CibouletteResourceSelector::Many(rs) => {
                CibouletteResourceSelector::Many(rs.into_iter().map(|x| x.into()).collect::<Vec<
                    CibouletteResource<
                        'request,
                        B,
                        CibouletteResourceIdentifierPermissive<'request>,
                    >,
                >>(
                ))
            }
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

impl<'request> CibouletteResourceSelectorBuilder<'request> {
    /// Build the resource selector from the builder
    pub fn build(
        self,
        bag: &CibouletteStore,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteResourceSelector<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
        CibouletteError,
    > {
        match self {
            CibouletteResourceSelectorBuilder::One(element) => Ok(CibouletteResourceSelector::One(
                element.build(bag, &intention)?,
            )),
            CibouletteResourceSelectorBuilder::Many(elements) => {
                let mut res: Vec<
                    CibouletteResource<
                        MessyJsonObjectValue<'request>,
                        CibouletteResourceIdentifierPermissive,
                    >,
                > = Vec::with_capacity(elements.len());

                for el in elements.into_iter() {
                    res.push(el.build(bag, &intention)?);
                }
                Ok(CibouletteResourceSelector::Many(res))
            }
        }
    }
}
