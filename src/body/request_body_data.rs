use super::*;

pub type CibouletteBodyDataBuilder<'request> =
    CibouletteOptionalData<CibouletteResourceSelectorBuilder<'request>>;
pub type CibouletteBodyData<'request, 'store, I, B> =
    CibouletteOptionalData<CibouletteResourceSelector<'request, 'store, B, I>>;

impl<'request> Default for CibouletteBodyDataBuilder<'request> {
    fn default() -> Self {
        CibouletteBodyDataBuilder::Null(false)
    }
}
// CibouletteResourceSelector<'request, CibouletteResourceIdentifierPermissive<'request>>

impl<'request, 'store, I, B> Default for CibouletteBodyData<'request, 'store, I, B> {
    fn default() -> Self {
        CibouletteBodyData::Null(false)
    }
}

impl<'request> CibouletteBodyDataBuilder<'request> {
    pub fn build<'store>(
        self,
        bag: &'store CibouletteStore<'store>,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteBodyData<
            'request,
            'store,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'store>,
        >,
        CibouletteError,
    >
    where
        'request: 'store,
    {
        match self {
            CibouletteBodyDataBuilder::Object(x) => {
                Ok(CibouletteBodyData::Object(x.build(bag, intention)?))
            }
            CibouletteBodyDataBuilder::Null(x) => Ok(CibouletteBodyData::Null(x)),
        }
    }
}
