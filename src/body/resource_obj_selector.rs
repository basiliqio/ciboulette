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
pub enum CibouletteResourceSelector<'request, 'store, B, T> {
    One(CibouletteResource<'request, 'store, B, T>),
    Many(Vec<CibouletteResource<'request, 'store, B, T>>),
}

impl<'request, 'store, B>
    TryFrom<
        CibouletteResourceSelector<
            'request,
            'store,
            B,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    > for CibouletteResourceSelector<'request, 'store, B, CibouletteResourceIdentifier<'request>>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResourceSelector<
            'request,
            'store,
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
                    Vec<
                        CibouletteResource<
                            'request,
                            'store,
                            B,
                            CibouletteResourceIdentifier<'request>,
                        >,
                    >,
                    CibouletteError,
                >>()?,
            )),
        }
    }
}

impl<'request, 'store, B>
    From<CibouletteResourceSelector<'request, 'store, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResourceSelector<
        'request,
        'store,
        B,
        CibouletteResourceIdentifierPermissive<'request>,
    >
{
    fn from(
        value: CibouletteResourceSelector<
            'request,
            'store,
            B,
            CibouletteResourceIdentifier<'request>,
        >,
    ) -> Self {
        match value {
            CibouletteResourceSelector::One(r) => CibouletteResourceSelector::One(r.into()),
            CibouletteResourceSelector::Many(rs) => {
                CibouletteResourceSelector::Many(rs.into_iter().map(|x| x.into()).collect::<Vec<
                    CibouletteResource<
                        'request,
                        'store,
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
    pub fn build<'store>(
        self,
        bag: &'store CibouletteStore<'store>,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteResourceSelector<
            'request,
            'store,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
        CibouletteError,
    >
    where
        'request: 'store,
    {
        match self {
            CibouletteResourceSelectorBuilder::One(element) => Ok(CibouletteResourceSelector::One(
                element.build(bag, &intention)?,
            )),
            CibouletteResourceSelectorBuilder::Many(elements) => {
                let mut res: Vec<
                    CibouletteResource<
                        MessyJsonObjectValue<'store>,
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

impl<'request, 'store, T>
    CibouletteResourceSelector<'request, 'store, MessyJsonObjectValue<'store>, T>
{
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
