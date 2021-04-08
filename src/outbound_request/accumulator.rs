use super::*;

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulator<'response, 'store, B> {
    pub(super) main_data: BTreeMap<
        CibouletteResourceIdentifier<'response>,
        CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'response, 'store, B>>,
    max_elements: Option<usize>,
    only_ids: bool,
}

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulatorSettings {
    max_elements: Option<usize>,
    only_ids: bool,
}

impl CibouletteOutboundRequestDataAccumulatorSettings {
    pub fn new(max_elements: Option<usize>, only_ids: bool) -> Self {
        CibouletteOutboundRequestDataAccumulatorSettings {
            only_ids,
            max_elements,
        }
    }
}

impl<'request, 'store> From<&dyn CibouletteInboundRequestCommons<'request, 'store>>
    for CibouletteOutboundRequestDataAccumulatorSettings
{
    fn from(inbound_request: &dyn CibouletteInboundRequestCommons<'request, 'store>) -> Self {
        match inbound_request.expected_response_type() {
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single) => {
                CibouletteOutboundRequestDataAccumulatorSettings::new(Some(1), false)
            }
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple) => {
                CibouletteOutboundRequestDataAccumulatorSettings::new(None, false)
            }
            CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                CibouletteOutboundRequestDataAccumulatorSettings::new(Some(1), true)
            }
            CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                CibouletteOutboundRequestDataAccumulatorSettings::new(None, true)
            }
            CibouletteResponseRequiredType::None => {
                CibouletteOutboundRequestDataAccumulatorSettings::new(Some(0), false)
            }
        }
    }
}

impl<'response, 'store, B> From<CibouletteOutboundRequestDataAccumulatorSettings>
    for CibouletteOutboundRequestDataAccumulator<'response, 'store, B>
{
    fn from(settings: CibouletteOutboundRequestDataAccumulatorSettings) -> Self {
        CibouletteOutboundRequestDataAccumulator {
            max_elements: settings.max_elements,
            only_ids: settings.only_ids,
            main_data: BTreeMap::new(),
            included_data: Vec::new(),
        }
    }
}

pub(super) struct CibouletteOutboundRequestExtractedData<'request, 'store, B> {
    pub main_data: CibouletteOptionalData<
        CibouletteResourceSelector<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
    >,
    pub included_data:
        Vec<CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>>,
}

impl<'response, 'store, B> CibouletteOutboundRequestDataAccumulator<'response, 'store, B> {
    pub fn new(max_elements: Option<usize>, only_ids: bool) -> Self {
        CibouletteOutboundRequestDataAccumulator {
            main_data: BTreeMap::default(),
            included_data: Vec::default(),
            max_elements,
            only_ids,
        }
    }

    /// Extract the accumulated data
    pub fn extract<'request>(
        self,
        inbound_request: &dyn CibouletteInboundRequestCommons<'request, 'store>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'response, 'store, B>, CibouletteError> {
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
            CibouletteResourceIdentifier<'response>,
            CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
        >,
        included_data: Vec<CibouletteResponseElement<'response, 'store, B>>,
    ) -> Result<
        Vec<CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>>,
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
                    CibouletteResource<
                        'response,
                        'store,
                        B,
                        CibouletteResourceIdentifier<'response>,
                    >,
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
            CibouletteResourceIdentifier<'response>,
            CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
        >,
        inbound_request: &dyn CibouletteInboundRequestCommons,
    ) -> CibouletteOptionalData<
        CibouletteResourceSelector<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
    > {
        let body_data: CibouletteBodyData<
            'response,
            'store,
            CibouletteResourceIdentifier<'response>,
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
        el: CibouletteResponseElement<'response, 'store, B>,
        main_data: &mut BTreeMap<
            CibouletteResourceIdentifier<'response>,
            CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
        >,
    ) -> Result<
        CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier<'response>>,
        CibouletteError,
    > {
        let related = match el.related() {
            Some(x) => x,
            None => todo!(),
        };
        if let Some(main_el) = main_data.get_mut(&related) {
            insert_relationships_into_existing(main_el, el.identifier().clone())?;
        }
        let resource = CibouletteResource::<B, CibouletteResourceIdentifier<'response>> {
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
fn insert_relationships_into_existing<'response, 'store, B>(
    obj: &mut CibouletteResource<'response, 'store, B, CibouletteResourceIdentifier>,
    identifier: CibouletteResourceIdentifier<'response>,
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
