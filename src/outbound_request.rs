use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteOutboundRequestBuilder<'a> {
    inbound_request: CibouletteInboundRequest<'a>,
    body: Option<CibouletteBody<'a>>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteOutboundRequest<'a> {
    pub inbound_request: CibouletteInboundRequest<'a>,
    pub body: Option<CibouletteBody<'a>>,
}

impl<'a> CibouletteOutboundRequestBuilder<'a> {
    pub fn new(
        inbound_request: CibouletteInboundRequest<'a>,
        body: Option<CibouletteBody<'a>>,
    ) -> Self {
        CibouletteOutboundRequestBuilder {
            inbound_request,
            body,
        }
    }

    pub fn build(
        self,
        _store: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteOutboundRequest<'a>, CibouletteError> {
        Ok(CibouletteOutboundRequest {
            inbound_request: self.inbound_request,
            body: self.body,
        })
    }
}
