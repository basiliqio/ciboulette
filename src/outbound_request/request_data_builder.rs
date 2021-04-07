use super::*;

/// A builder structure for [CibouletteOutboundRequest](CibouletteOutboundRequest)
pub struct CibouletteOutboundRequestDataBuilder<'request, 'store, B, I>
where
    I: IntoIterator<Item = CibouletteResponseElement<'request, 'store, B>>,
{
    /// The inbound request is made from
    inbound_request: &'request dyn CibouletteInboundRequestCommons<'request, 'store>,
    /// An iterator over its elements
    elements: I,
}

impl<'request, 'store, B, I> CibouletteOutboundRequestDataBuilder<'request, 'store, B, I>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'request, 'store, B>>,
{
    /// Create a new builder from its parts
    pub fn new(
        inbound_request: &'request dyn CibouletteInboundRequestCommons<'request, 'store>,
        elements: I,
    ) -> Self {
        CibouletteOutboundRequestDataBuilder {
            inbound_request,
            elements,
        }
    }

    fn build_body(
        inbound_request: &'request dyn CibouletteInboundRequestCommons<'request, 'store>,
        elements: I,
    ) -> Result<
        CibouletteBody<'request, 'store, CibouletteResourceIdentifier<'request>, B>,
        CibouletteError,
    > {
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

    /// Build the outbound request
    pub fn build(self) -> Result<CibouletteOutboundRequest<'request, 'store, B>, CibouletteError> {
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
