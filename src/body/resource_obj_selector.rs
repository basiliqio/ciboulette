use super::*;

/// ## Builder object for [CibouletterResourceSelector](CibouletterResourceSelector)
#[derive(Debug)]
pub struct CibouletteResourceSelectorBuilder<'request>(
    CibouletteSelector<CibouletteResourceBuilder<'request>>,
);

ciboulette_selector_utils!(CibouletteResourceSelectorBuilder, CibouletteResourceBuilder, 'request);
/// ## A selector between a single or multiple `json:api` [resource](https://jsonapi.org/format/#document-resource-objects) objects
#[derive(Debug, Clone, Serialize)]
pub struct CibouletteResourceSelector<'request, B, T>(
    CibouletteSelector<CibouletteResource<'request, B, T>>,
);

ciboulette_selector_utils!(CibouletteResourceSelector, CibouletteResource, 'request, B, T);

impl<'de> serde::Deserialize<'de> for CibouletteResourceSelectorBuilder<'de> {
    fn deserialize<D>(deserializer: D) -> Result<CibouletteResourceSelectorBuilder<'de>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let selector =
            CibouletteSelector::<CibouletteResourceBuilder<'de>>::deserialize(deserializer)?;
        Ok(CibouletteResourceSelectorBuilder::new(selector))
    }
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
        match value.take() {
            CibouletteSelector::Single(r) => Ok(CibouletteResourceSelector::new(
                CibouletteSelector::Single(r.try_into()?),
            )),
            CibouletteSelector::Multi(rs) => Ok(CibouletteResourceSelector::new(
                CibouletteSelector::Multi(rs.into_iter().map(|x| x.try_into()).collect::<Result<
                    Vec<CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>>,
                    CibouletteError,
                >>(
                )?),
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
        match value.take() {
            CibouletteSelector::Single(r) => {
                CibouletteResourceSelector::new(CibouletteSelector::Single(r.into()))
            }
            CibouletteSelector::Multi(rs) => CibouletteResourceSelector::new(
                CibouletteSelector::Multi(rs.into_iter().map(|x| x.into()).collect::<Vec<
                    CibouletteResource<
                        'request,
                        B,
                        CibouletteResourceIdentifierPermissive<'request>,
                    >,
                >>()),
            ),
        }
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
        match self.take() {
            CibouletteSelector::Single(element) => Ok(CibouletteResourceSelector::new(
                CibouletteSelector::Single(element.build(bag, &intention)?),
            )),
            CibouletteSelector::Multi(elements) => {
                let mut res: Vec<
                    CibouletteResource<
                        MessyJsonObjectValue<'request>,
                        CibouletteResourceIdentifierPermissive,
                    >,
                > = Vec::with_capacity(elements.len());

                for el in elements.into_iter() {
                    res.push(el.build(bag, &intention)?);
                }
                Ok(CibouletteResourceSelector::new(CibouletteSelector::Multi(
                    res,
                )))
            }
        }
    }
}
