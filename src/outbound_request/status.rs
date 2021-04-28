use super::*;

/// The status a response should send
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CibouletteResponseStatus {
    /// HTTP 200
    Ok,
    /// HTTP 204
    OkEmpty,
    /// HTTP 202
    OkAsync,
    /// HTTP 201
    Created,
    /// HTTP 403
    Unsupported,
    /// HTTP 404
    NotFound,
    /// HTTP 409
    Conflict,
}

impl CibouletteResponseStatus {
    /// Get a response status for a given request type and data.
    ///
    /// Applicable only if the request was a success
    pub fn get_status_for_ok_response<'request, 'response, B>(
        request: &dyn CibouletteInboundRequestCommons<'request>,
        response_body: &CibouletteResponseBody<'response, B>,
    ) -> Self {
        match (request.intention(), response_body.data()) {
            (CibouletteIntention::Create, CibouletteOptionalData::Object(_)) => {
                CibouletteResponseStatus::Created
            }
            (CibouletteIntention::Create, CibouletteOptionalData::Null(_)) => {
                CibouletteResponseStatus::Ok
                // TODO : In the future, CibouletteResponseStatus::OkEmpty should be used
            }
            // TODO :   In the future, (CibouletteIntention::Delete, _) => CibouletteResponseStatus::OkEmpty should be used,
            (CibouletteIntention::Delete, _) => CibouletteResponseStatus::Ok,
            (CibouletteIntention::Read, CibouletteOptionalData::Object(_)) => {
                CibouletteResponseStatus::Ok
            }
            (CibouletteIntention::Read, CibouletteOptionalData::Null(_)) => {
                CibouletteResponseStatus::NotFound
            }
            (CibouletteIntention::Update, CibouletteOptionalData::Object(_)) => {
                CibouletteResponseStatus::Ok
            }
            (CibouletteIntention::Update, CibouletteOptionalData::Null(_)) => {
                CibouletteResponseStatus::Ok
                // TODO : In the future, CibouletteResponseStatus::OkEmpty should be used
            }
        }
    }
}
