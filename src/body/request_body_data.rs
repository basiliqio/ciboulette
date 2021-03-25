use super::*;

pub type CibouletteBodyDataBuilder<'a> =
    CibouletteOptionalData<CibouletteResourceSelectorBuilder<'a>>;
pub type CibouletteBodyData<'a, B> = CibouletteOptionalData<
    CibouletteResourceSelector<'a, B, CibouletteResourceIdentifierPermissive<'a>>,
>;

impl<'a> Default for CibouletteBodyDataBuilder<'a> {
    fn default() -> Self {
        CibouletteBodyDataBuilder::Null(false)
    }
}
// CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>

impl<'a, B> Default for CibouletteBodyData<'a, B> {
    fn default() -> Self {
        CibouletteBodyData::Null(false)
    }
}

impl<'a> CibouletteBodyDataBuilder<'a> {
    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
        intention: &CibouletteIntention,
    ) -> Result<CibouletteBodyData<'a, MessyJsonObjectValue<'a>>, CibouletteError> {
        match self {
            CibouletteBodyDataBuilder::Object(x) => {
                Ok(CibouletteBodyData::Object(x.build(bag, intention)?))
            }
            CibouletteBodyDataBuilder::Null(x) => Ok(CibouletteBodyData::Null(x)),
        }
    }
}
