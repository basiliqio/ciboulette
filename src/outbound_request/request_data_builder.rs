use super::*;

/// A builder structure for [CibouletteOutboundRequest](CibouletteOutboundRequest)
pub struct CibouletteOutboundRequestDataBuilder<'request, 'response, B, I>
where
    I: IntoIterator<Item = CibouletteResponseElement<'response, B>>,
{
    /// The inbound request is made from
    inbound_request: &'request dyn CibouletteInboundRequestCommons<'request>,
    /// An iterator over its elements
    elements: I,

    marker: std::marker::PhantomData<&'response str>,
}

impl<'request, 'response, B, I> CibouletteOutboundRequestDataBuilder<'request, 'response, B, I>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'response, B>>,
{
    /// Create a new builder from its parts
    pub fn new(
        inbound_request: &'request dyn CibouletteInboundRequestCommons<'request>,
        elements: I,
    ) -> Self {
        CibouletteOutboundRequestDataBuilder {
            inbound_request,
            elements,
            marker: std::marker::PhantomData::default(),
        }
    }

    fn build_body(
        store: &CibouletteStore,
        inbound_request: &'request dyn CibouletteInboundRequestCommons<'request>,
        elements: I,
    ) -> Result<
        CibouletteBody<'response, CibouletteResourceResponseIdentifier<'response>, B>,
        CibouletteError,
    > {
        let acc_settings = CibouletteOutboundRequestDataAccumulatorSettings::from(inbound_request);
        let acc = element::fold_elements(elements, acc_settings, inbound_request)?;
        let extracted_data = acc.extract(store, inbound_request)?;
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
    pub fn build(
        self,
        store: &CibouletteStore,
    ) -> Result<CibouletteOutboundRequest<'response, B>, CibouletteError> {
        let body = Self::build_body(store, self.inbound_request, self.elements)?;
        Ok(CibouletteOutboundRequest {
            status: CibouletteResponseStatus::get_status_for_ok_response(
                self.inbound_request,
                &body,
            ),
            body,
        })
    }
}
