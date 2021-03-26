use super::*;
use getset::CopyGetters;

#[derive(Debug, Getters, CopyGetters, Serialize)]
pub struct CibouletteOutboundRequest<'a, B> {
    #[serde(flatten)]
    #[getset(get = "pub")]
    pub body: CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>,
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing)]
    pub status: CibouletteResponseStatus,
}
