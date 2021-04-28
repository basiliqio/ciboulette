use super::*;
use getset::CopyGetters;

/// A response, built from a request.
#[derive(Debug, Getters, CopyGetters, Serialize)]
pub struct CibouletteResponse<'response, B> {
    /// The body of the response.
    #[serde(flatten)]
    #[getset(get = "pub")]
    pub body: CibouletteResponseBody<'response, B>,
    /// The status of the response
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing)]
    pub status: CibouletteResponseStatus,
}
