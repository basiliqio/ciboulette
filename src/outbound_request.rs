use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteOutboundRequestBuilder<'a, B> {
    inbound_request: CibouletteInboundRequest<'a>,
    body: Option<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>>,
    status: CibouletteResponseStatus,
}

#[derive(Debug, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteOutboundRequest<'a, B> {
    #[serde(skip_serializing)]
    pub inbound_request: CibouletteInboundRequest<'a>,
    #[serde(flatten)]
    pub body: Option<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>>,
    #[serde(skip_serializing)]
    pub status: CibouletteResponseStatus,
}

impl<'a, B> CibouletteOutboundRequestBuilder<'a, B>
where
    B: Serialize,
{
    pub fn new(
        inbound_request: CibouletteInboundRequest<'a>,
        body: Option<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>>,
        status: CibouletteResponseStatus,
    ) -> Self {
        CibouletteOutboundRequestBuilder {
            inbound_request,
            body,
            status,
        }
    }

    pub fn build(
        self,
        _store: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteOutboundRequest<'a, B>, CibouletteError> {
        Ok(CibouletteOutboundRequest {
            inbound_request: self.inbound_request,
            body: self.body,
            status: self.status,
        })
    }
}
