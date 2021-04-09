use super::*;
use getset::CopyGetters;

/// An outbound response, built from an inbound request.
#[derive(Debug, Getters, CopyGetters, Serialize)]
pub struct CibouletteOutboundRequest<'response, B> {
    /// The body of the response.
    #[serde(flatten)]
    #[getset(get = "pub")]
    pub body: CibouletteBody<'response, CibouletteResourceIdentifier<'response>, B>,
    /// The status of the response
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing)]
    pub status: CibouletteResponseStatus,
}
