use super::*;

pub type CibouletteBodyDataBuilder<'request> =
    CibouletteOptionalData<CibouletteResourceSelectorBuilder<'request>>;
pub type CibouletteBodyData<'request, I, B> =
    CibouletteOptionalData<CibouletteResourceSelector<'request, B, I>>;

// CibouletteResourceSelector<'request, CibouletteResourceIdentifierPermissive<'request>>

impl<'request> CibouletteBodyDataBuilder<'request> {
    pub fn build(
        self,
        bag: &CibouletteStore,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteBodyData<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
        CibouletteError,
    > {
        match self {
            CibouletteBodyDataBuilder::Object(x) => {
                Ok(CibouletteBodyData::Object(x.build(bag, intention)?))
            }
            CibouletteBodyDataBuilder::Null(x) => Ok(CibouletteBodyData::Null(x)),
        }
    }
}
