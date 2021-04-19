use super::*;

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulator<'response, B> {
    pub(super) main_data: BTreeMap<
        CibouletteResourceResponseIdentifier<'response>,
        CibouletteResponseResource<'response, B>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'response, B>>,
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

impl<'request> From<&dyn CibouletteInboundRequestCommons<'request>>
    for CibouletteOutboundRequestDataAccumulatorSettings
{
    fn from(inbound_request: &dyn CibouletteInboundRequestCommons<'request>) -> Self {
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

impl<'response, B> From<CibouletteOutboundRequestDataAccumulatorSettings>
    for CibouletteOutboundRequestDataAccumulator<'response, B>
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

pub(super) struct CibouletteOutboundRequestExtractedData<'request, B> {
    pub main_data: CibouletteOptionalData<CibouletteResponseResourceSelector<'request, B>>,
    pub included_data: Vec<CibouletteResponseResource<'request, B>>,
}

impl<'response, B> CibouletteOutboundRequestDataAccumulator<'response, B> {
    /// Extract the accumulated data
    pub fn extract<'request>(
        self,
        store: &CibouletteStore,
        inbound_request: &dyn CibouletteInboundRequestCommons<'request>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'response, B>, CibouletteError> {
        let mut main_data = self.main_data;
        let included_data = Self::extract_included_data(store, &mut main_data, self.included_data)?;
        let body_data = Self::extract_main_data(store, main_data, inbound_request);
        Ok(CibouletteOutboundRequestExtractedData {
            main_data: body_data,
            included_data,
        })
    }

    fn extract_included_data(
        store: &CibouletteStore,
        main_data: &mut BTreeMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        included_data: Vec<CibouletteResponseElement<'response, B>>,
    ) -> Result<Vec<CibouletteResponseResource<'response, B>>, CibouletteError> {
        match included_data.len() {
            0 => Ok(vec![]),
            1 => {
                let el = included_data.into_iter().next().unwrap();
                let resource = Self::insert_included_data_as_relationships(store, el, main_data)?;
                Ok(vec![resource])
            }
            _ => {
                let mut res: Vec<CibouletteResponseResource<'response, B>> =
                    Vec::with_capacity(included_data.len());
                for el in included_data.into_iter() {
                    let resource =
                        Self::insert_included_data_as_relationships(store, el, main_data)?;
                    res.push(resource)
                }
                Ok(res)
            }
        }
    }

    fn extract_main_data(
        store: &CibouletteStore,
        main_data: BTreeMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        inbound_request: &dyn CibouletteInboundRequestCommons,
    ) -> CibouletteOptionalData<CibouletteResponseResourceSelector<'response, B>> {
        let body_data: CibouletteResponseBodyData<'response, B> =
            match inbound_request.expected_response_type() {
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single)
                | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                    match main_data.into_iter().next() {
                        Some((_, x)) => CibouletteOptionalData::Object(
                            CibouletteResponseResourceSelector::One(x),
                        ),
                        None => CibouletteOptionalData::Null(true),
                    }
                }
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple)
                | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                    let mut res = Vec::with_capacity(main_data.len());
                    for (_, el) in main_data.into_iter() {
                        res.push(el);
                    }
                    CibouletteOptionalData::Object(CibouletteResponseResourceSelector::Many(res))
                }
                CibouletteResponseRequiredType::None => CibouletteOptionalData::Null(true),
            };
        body_data
    }
    fn insert_included_data_as_relationships(
        store: &CibouletteStore,
        el: CibouletteResponseElement<'response, B>,
        main_data: &mut BTreeMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
    ) -> Result<CibouletteResponseResource<'response, B>, CibouletteError> {
        let related = match el.related() {
            Some(x) => x,
            None => {
                return Err(CibouletteError::UnknownError(
                    "No related data for orphaned row".into(),
                ))
            }
        };
        if let Some(main_el) = main_data.get_mut(related.element()) {
            insert_relationships_into_existing(main_el, el.identifier().clone(), related.alias())?;
        }
        let resource = CibouletteResponseResource::<B> {
            type_: el.type_,
            identifier: el.identifier,
            attributes: el.data,
            relationships: BTreeMap::default(),
            links: Option::default(),
        };
        Ok(resource)
    }
}

/// Inserts into an existing relationships a new entry, updating its format if necessary
fn insert_relationships_into_existing<'response, B>(
    obj: &mut CibouletteResponseResource<'response, B>,
    alias_identifier: CibouletteResourceResponseIdentifier<'response>,
    alias_str: &ArcStr,
) -> Result<(), CibouletteError> {
    let alias_arc = obj
        .type_()
        .relationships()
        .get_key_value(alias_str)
        .map(|(k, _)| k.clone())
        .ok_or_else(|| {
            CibouletteError::UnknownRelationship(
                obj.type_().name().to_string(),
                alias_str.to_string(),
            )
        })?;
    if let Some(rel) = obj.relationships_mut().get_mut(&alias_arc) {
        let data = rel.data_mut();
        match data {
            CibouletteOptionalData::Object(CibouletteResourceResponseIdentifierSelector::One(
                existing_id,
            )) => {
                let res = CibouletteOptionalData::Object(
                    CibouletteResourceResponseIdentifierSelector::Many(vec![
                        existing_id.clone(),
                        alias_identifier.clone(),
                    ]),
                );
                *data = res;
            }
            CibouletteOptionalData::Null(_) => {
                let res = CibouletteOptionalData::Object(
                    CibouletteResourceResponseIdentifierSelector::One(alias_identifier.clone()),
                );
                *data = res;
            }
            CibouletteOptionalData::Object(CibouletteResourceResponseIdentifierSelector::Many(
                existing_ids,
            )) => {
                existing_ids.push(alias_identifier.clone());
            }
        }
    } else {
        obj.relationships_mut().insert(
            alias_arc,
            CibouletteResponseRelationshipObject {
                // TODO links
                data: CibouletteOptionalData::Object(
                    CibouletteResourceResponseIdentifierSelector::One(alias_identifier.clone()),
                ),
                ..Default::default()
            },
        );
    }
    Ok(())
}
