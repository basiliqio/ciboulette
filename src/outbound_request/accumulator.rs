use super::*;

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulator<'request, 'store, B> {
    pub(super) main_data: BTreeMap<
        CibouletteResourceIdentifier<'request>,
        CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'request, 'store, B>>,
    max_elements: Option<usize>,
    only_ids: bool,
}

pub(super) struct CibouletteOutboundRequestExtractedData<'request, 'store, B> {
    pub main_data: CibouletteOptionalData<
        CibouletteResourceSelector<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
    >,
    pub included_data:
        Vec<CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>>,
}

impl<'request, 'store, B> CibouletteOutboundRequestDataAccumulator<'request, 'store, B> {
    pub fn new(max_elements: Option<usize>, only_ids: bool) -> Self {
        CibouletteOutboundRequestDataAccumulator {
            main_data: BTreeMap::default(),
            included_data: Vec::default(),
            max_elements,
            only_ids,
        }
    }

    /// Init an accumulator from an existing request
    pub fn init_from_request(
        inbound_request: &dyn CibouletteInboundRequestCommons<'request, 'store>,
    ) -> CibouletteOutboundRequestDataAccumulator<'request, 'store, B> {
        let acc: CibouletteOutboundRequestDataAccumulator<'request, 'store, B> =
            match inbound_request.expected_response_type() {
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single) => {
                    CibouletteOutboundRequestDataAccumulator::new(Some(1), false)
                }
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple) => {
                    CibouletteOutboundRequestDataAccumulator::new(None, false)
                }
                CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                    CibouletteOutboundRequestDataAccumulator::new(Some(1), true)
                }
                CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                    CibouletteOutboundRequestDataAccumulator::new(None, true)
                }
                CibouletteResponseRequiredType::None => {
                    CibouletteOutboundRequestDataAccumulator::new(Some(0), false)
                }
            };
        acc
    }

    /// Extract the accumulated data
    pub fn extract(
        self,
        inbound_request: &dyn CibouletteInboundRequestCommons<'request, 'store>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'request, 'store, B>, CibouletteError> {
        let mut main_data = self.main_data;
        let included_data = Self::extract_included_data(&mut main_data, self.included_data)?;
        let body_data = Self::extract_main_data(main_data, inbound_request);
        Ok(CibouletteOutboundRequestExtractedData {
            main_data: body_data,
            included_data,
        })
    }

    fn extract_included_data(
        main_data: &mut BTreeMap<
            CibouletteResourceIdentifier<'request>,
            CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
        >,
        included_data: Vec<CibouletteResponseElement<'request, 'store, B>>,
    ) -> Result<
        Vec<CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>>,
        CibouletteError,
    > {
        match included_data.len() {
            0 => Ok(vec![]),
            1 => {
                let el = included_data.into_iter().next().unwrap();
                let resource = Self::insert_included_data_as_relationships(el, main_data)?;
                Ok(vec![resource])
            }
            _ => {
                let mut res: Vec<
                    CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
                > = Vec::with_capacity(included_data.len());
                for el in included_data.into_iter() {
                    let resource = Self::insert_included_data_as_relationships(el, main_data)?;
                    res.push(resource)
                }
                Ok(res)
            }
        }
    }

    fn extract_main_data(
        main_data: BTreeMap<
            CibouletteResourceIdentifier<'request>,
            CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
        >,
        inbound_request: &dyn CibouletteInboundRequestCommons,
    ) -> CibouletteOptionalData<
        CibouletteResourceSelector<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
    > {
        let body_data: CibouletteBodyData<
            'request,
            'store,
            CibouletteResourceIdentifier<'request>,
            B,
        > = match inbound_request.expected_response_type() {
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single)
            | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                match main_data.into_iter().next() {
                    Some((_, x)) => {
                        CibouletteOptionalData::Object(CibouletteResourceSelector::One(x))
                    }
                    None => CibouletteOptionalData::Null(true),
                }
            }
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple)
            | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                let mut res = Vec::with_capacity(main_data.len());
                for (_, el) in main_data.into_iter() {
                    res.push(el);
                }
                CibouletteOptionalData::Object(CibouletteResourceSelector::Many(res))
            }
            CibouletteResponseRequiredType::None => CibouletteOptionalData::Null(true),
        };
        body_data
    }
    fn insert_included_data_as_relationships(
        el: CibouletteResponseElement<'request, 'store, B>,
        main_data: &mut BTreeMap<
            CibouletteResourceIdentifier<'request>,
            CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
        >,
    ) -> Result<
        CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
        CibouletteError,
    > {
        let related = match el.related() {
            Some(x) => x,
            None => todo!(),
        };
        if let Some(main_el) = main_data.get_mut(&related) {
            insert_relationships_into_existing(main_el, el.identifier().clone())?;
        }
        let resource = CibouletteResource::<B, CibouletteResourceIdentifier<'request>> {
            type_: el.type_,
            identifier: el.identifier,
            attributes: el.data,
            relationships: BTreeMap::default(),
            links: Option::default(),
            meta: None, //FIXME
        };
        Ok(resource)
    }
}

/// Inserts into an existing relationships a new entry, updating its format if necessary
fn insert_relationships_into_existing<'request, 'store, B>(
    obj: &mut CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier>,
    identifier: CibouletteResourceIdentifier<'request>,
) -> Result<(), CibouletteError> {
    let alias = obj.type_().get_alias(identifier.type_())?.clone();
    if let Some(rel) = obj.relationships_mut().get_mut(alias.as_str()) {
        let data = rel.data_mut();
        match data {
            CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(
                existing_id,
            )) => {
                let res =
                    CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(
                        vec![existing_id.clone(), identifier.clone()],
                    ));
                *data = res;
            }
            CibouletteOptionalData::Null(_) => {
                let res = CibouletteOptionalData::Object(
                    CibouletteResourceIdentifierSelector::One(identifier.clone()),
                );
                *data = res;
            }
            CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(
                existing_ids,
            )) => {
                existing_ids.push(identifier.clone());
            }
        }
    } else {
        obj.relationships_mut().insert(
            alias.clone(),
            CibouletteRelationshipObject {
                // TODO links
                data: CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(
                    identifier.clone(),
                )),
                ..Default::default()
            },
        );
    }
    Ok(())
}
