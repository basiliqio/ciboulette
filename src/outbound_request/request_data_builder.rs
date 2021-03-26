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
        ciboulette_store: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteBody<'a, CibouletteResourceIdentifier<'a>, B>, CibouletteError> {
        let acc = CibouletteOutboundRequestDataAccumulator::init_from_request(inbound_request);
        let acc = element::fold_elements(ciboulette_store, elements, acc, inbound_request)?;
        let extracted_data = acc.extract(inbound_request)?;
        Ok(CibouletteBody {
            data: extracted_data.main_data,
            errors: None,
            meta: inbound_request.meta().clone(), //FIXME,
            links: None,                          //TODO,
            jsonapi: None,                        //TODO
            included: extracted_data.included_data,
        })
    }

    // pub fn build(
    //     self,
    //     _store: &'a CibouletteStore<'a>,
    // ) -> Result<CibouletteOutboundRequest<'a, B>, CibouletteError> {
    // 	let mut global_iterator = self.elements.into_iter();
    // 	let main_data = {
    // 		let main_data_iter = global_iterator
    // 			.by_ref()
    // 			.filter(|x| x.type_() == self.inbound_request.path().main_type().name().as_str());
    // 		fold_rows(
    // 			ciboulette_store,
    // 			request,
    // 			main_data_iter,
    // 		)?
    // 	};
    // 	let included_data = {
    // 		let included_data_iter = global_iterator
    // 			.by_ref()
    // 			.filter(|x| x.type_() == self.inbound_request.path().main_type().name().as_str());
    // 		fold_rows(
    // 			ciboulette_store,
    // 			request,
    // 			included_data_iter,
    // 		)?
    // 	};
    // 	let body
    //     Ok(CibouletteOutboundRequest {
    //         body: self.body,
    //         status: self.status,
    //     })
    // }
}
