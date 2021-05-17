use super::*;

/// A builder structure for [CibouletteOutboundRequest](CibouletteOutboundRequest)
pub struct CibouletteResponseDataBuilder<'request, 'response, B, I>
where
    I: IntoIterator<Item = CibouletteResponseElement<'response, B>>,
{
    /// The inbound request is made from
    inbound_request: &'request dyn CibouletteRequestCommons<'request>,
    /// An iterator over its elements
    elements: I,
}

impl<'request, 'response, B, I> CibouletteResponseDataBuilder<'request, 'response, B, I>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'response, B>>,
{
    /// Create a new builder from its parts
    pub fn new(
        inbound_request: &'request dyn CibouletteRequestCommons<'request>,
        elements: I,
    ) -> Self {
        CibouletteResponseDataBuilder {
            inbound_request,
            elements,
        }
    }
    /// Build body of the outbound request
    fn build_body<'store>(
        config: &'store CibouletteConfig,
        inbound_request: &'request dyn CibouletteRequestCommons<'request>,
        elements: I,
    ) -> Result<CibouletteResponseBody<'response, B>, CibouletteError> {
        let acc_settings = CibouletteResponseDataAccumulatorSettings::from(inbound_request);
        let acc = element::fold_elements(elements, acc_settings)?;
        let extracted_data = acc.extract(config, inbound_request)?;
        let inner_link = links::build_link_for_response_root(config, inbound_request);
        let body_link = match inner_link {
            Some(inner_link) => Some(CibouletteBodyLink {
                inner_link,
                pagination: CibouletteBodyPagination::default(),
            }),
            None => None,
        };
        Ok(CibouletteResponseBody {
            data: extracted_data.main_data,
            errors: None,
            links: body_link,
            jsonapi: Some(CibouletteJsonApiVersion::new(Cow::Borrowed("1.0"))),
            included: extracted_data
                .included_data
                .into_iter()
                .map(|(_, v)| v)
                .collect(),
        })
    }

    /// Build the outbound request
    pub fn build<'store>(
        self,
        config: &'store CibouletteConfig,
    ) -> Result<CibouletteResponse<'response, B>, CibouletteError> {
        let body: CibouletteResponseBody<'response, B> =
            Self::build_body(config, self.inbound_request, self.elements)?;
        Ok(CibouletteResponse {
            status: CibouletteResponseStatus::get_status_for_ok_response(
                self.inbound_request,
                &body,
            ),
            body,
        })
    }
}
