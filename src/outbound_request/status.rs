use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CibouletteResponseStatus {
    Ok,
    OkEmpty,
    OkAsync,
    Created,
    Unsupported,
    NotFound,
    Conflict,
}

impl CibouletteResponseStatus {
    pub fn get_status_for_ok_response<'a, B>(
        request: &dyn CibouletteInboundRequestCommons<'a>,
        response_body: &CibouletteBody<CibouletteResourceIdentifier, B>,
    ) -> Self {
        match (request.intention(), response_body.data()) {
            (CibouletteIntention::Create, CibouletteOptionalData::Object(_)) => {
                CibouletteResponseStatus::Created
            }
            (CibouletteIntention::Create, CibouletteOptionalData::Null(_)) => {
                CibouletteResponseStatus::OkEmpty
            }
            (CibouletteIntention::Delete, _) => CibouletteResponseStatus::OkEmpty,
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
                CibouletteResponseStatus::OkEmpty
            }
        }
    }
}
