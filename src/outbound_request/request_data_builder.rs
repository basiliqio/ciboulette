use super::*;

pub struct CibouletteOutboundRequestDataBuilder<'a, B, I>
where
    I: IntoIterator<Item = CibouletteResponseElement<'a, B>>,
{
    inbound_request: &'a dyn CibouletteInboundRequestCommons<'a>,
    elements: I,
}

impl<'a, B, I> CibouletteOutboundRequestDataBuilder<'a, B, I>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'a, B>>,
{
    pub fn new(inbound_request: &'a dyn CibouletteInboundRequestCommons<'a>, elements: I) -> Self {
        CibouletteOutboundRequestDataBuilder {
            inbound_request,
            elements,
        }
    }

    fn build_body(
        inbound_request: &dyn CibouletteInboundRequestCommons<'a>,
        elements: I,
    ) -> Result<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>, CibouletteError> {
        let acc = CibouletteOutboundRequestDataAccumulator::init_from_request(inbound_request);
        let acc = element::fold_elements(elements, acc, inbound_request)?;
        let extracted_data = acc.extract(inbound_request)?;
        Ok(CibouletteBody {
            data: extracted_data.main_data,
            errors: None,
            meta: inbound_request.meta().clone(), //FIXME,
            links: None,                          //TODO,
            jsonapi: Some(CibouletteJsonApiVersion::new(Cow::Borrowed("1.0"))),
            included: extracted_data.included_data,
        })
    }

    pub fn build(self) -> Result<CibouletteOutboundRequest<'a, B>, CibouletteError> {
        let body = Self::build_body(self.inbound_request, self.elements)?;
        Ok(CibouletteOutboundRequest {
            status: CibouletteResponseStatus::get_status_for_ok_response(
                self.inbound_request,
                &body,
            ),
            body,
        })
    }
}
