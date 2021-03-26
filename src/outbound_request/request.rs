use super::*;

#[derive(Debug, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteOutboundRequest<'a, B> {
    #[serde(flatten)]
    pub body: Option<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>>,
    #[serde(skip_serializing)]
    pub status: CibouletteResponseStatus,
}
