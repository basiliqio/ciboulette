use super::*;

pub type CibouletteBodyDataBuilder<'a> =
    CibouletteOptionalData<CibouletteResourceSelectorBuilder<'a>>;
pub type CibouletteBodyData<'a> = CibouletteOptionalData<
    CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>,
>;

impl<'a> Default for CibouletteBodyDataBuilder<'a> {
    fn default() -> Self {
        CibouletteBodyDataBuilder::Null(false)
    }
}
// CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>

impl<'a> Default for CibouletteBodyData<'a> {
    fn default() -> Self {
        CibouletteBodyData::Null(false)
    }
}

impl<'a> CibouletteBodyDataBuilder<'a> {
    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
        intention: &CibouletteIntention,
        main_type: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteBodyData<'a>, CibouletteError> {
        match self {
            CibouletteBodyDataBuilder::Object(x) => Ok(CibouletteBodyData::Object(
                x.build(bag, intention, &main_type)?,
            )),
            CibouletteBodyDataBuilder::Null(x) => Ok(CibouletteBodyData::Null(x)),
        }
    }
}
