use super::*;
use element::CibouletteResponseElementAlias;

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulator<'response, B> {
    pub(super) main_data: indexmap::IndexMap<
        CibouletteResourceResponseIdentifier<'response>,
        CibouletteResponseResource<'response, B>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'response, B>>,
    settings: CibouletteOutboundRequestDataAccumulatorSettings,
}

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulatorSettings {
    max_elements: Option<usize>,
    only_ids: bool,
    main_type: Arc<CibouletteResourceType>,
    include_rels: Option<CibouletteResourceRelationshipDetails>,
}

impl CibouletteOutboundRequestDataAccumulatorSettings {
    pub fn new(
        main_type: Arc<CibouletteResourceType>,
        max_elements: Option<usize>,
        only_ids: bool,
        include_rels: Option<CibouletteResourceRelationshipDetails>,
    ) -> Self {
        CibouletteOutboundRequestDataAccumulatorSettings {
            only_ids,
            max_elements,
            main_type,
            include_rels,
        }
    }
}

impl<'request> From<&dyn CibouletteInboundRequestCommons<'request>>
    for CibouletteOutboundRequestDataAccumulatorSettings
{
    fn from(inbound_request: &dyn CibouletteInboundRequestCommons<'request>) -> Self {
        let (max_element, only_ids) = match inbound_request.expected_response_type() {
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single) => {
                (Some(1), false)
            }
            CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple) => {
                (None, false)
            }
            CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                (Some(1), true)
            }
            CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                (None, true)
            }
            CibouletteResponseRequiredType::None => (Some(0), false),
        };
        let include_rels = match inbound_request.path() {
            CiboulettePath::TypeIdRelationship(_, _, y) => Some(y.clone()),
            _ => None,
        };
        CibouletteOutboundRequestDataAccumulatorSettings::new(
            inbound_request.expected_type().clone(),
            max_element,
            only_ids,
            include_rels,
        )
    }
}

impl<'response, B> From<CibouletteOutboundRequestDataAccumulatorSettings>
    for CibouletteOutboundRequestDataAccumulator<'response, B>
{
    fn from(settings: CibouletteOutboundRequestDataAccumulatorSettings) -> Self {
        CibouletteOutboundRequestDataAccumulator {
            settings,
            main_data: IndexMap::new(),
            included_data: Vec::new(),
        }
    }
}

pub(super) struct CibouletteOutboundRequestExtractedData<'request, B> {
    pub main_data: CibouletteOptionalData<CibouletteResponseResourceSelector<'request, B>>,
    pub included_data: BTreeMap<
        CibouletteResourceResponseIdentifier<'request>,
        CibouletteResponseResource<'request, B>,
    >,
}

impl<'response, B> CibouletteOutboundRequestDataAccumulator<'response, B> {
    /// Extract the accumulated data
    pub fn extract<'request>(
        self,
        inbound_request: &dyn CibouletteInboundRequestCommons<'request>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'response, B>, CibouletteError> {
        let settings = self.settings;
        let mut main_data = self.main_data;
        match settings.include_rels() {
            None => {
                let included_data = Self::extract_included_data(
                    inbound_request.expected_type(),
                    &mut main_data,
                    self.included_data,
                )?;
                let body_data = Self::extract_main_data(main_data, inbound_request);
                Ok(CibouletteOutboundRequestExtractedData {
                    main_data: body_data,
                    included_data,
                })
            }
            Some(rel) => {
                let main_data = self
                    .included_data
                    .into_iter()
                    .filter_map(|x| {
                        x.related
                            .and_then(|x| match *x.rel_chain().as_slice() == [rel.clone()] {
                                true => {
                                    let res = CibouletteResponseResource {
                                        type_: rel.related_type().clone(),
                                        identifier: x.element,
                                        attributes: None,
                                        relationships: BTreeMap::default(),
                                        links: None,
                                    };
                                    Some((res.identifier().clone(), res))
                                }
                                false => None,
                            })
                    })
                    .collect();
                let body_data = Self::extract_main_data(main_data, inbound_request);
                Ok(CibouletteOutboundRequestExtractedData {
                    main_data: body_data,
                    included_data: BTreeMap::default(),
                })
            }
        }
    }

    fn extract_included_data(
        base_type: &Arc<CibouletteResourceType>,
        main_data: &mut IndexMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        included_data: Vec<CibouletteResponseElement<'response, B>>,
    ) -> Result<
        BTreeMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        CibouletteError,
    > {
        let mut res = BTreeMap::new();
        let mut late_linking: Vec<(
            CibouletteResourceResponseIdentifier,
            CibouletteResponseElementAlias,
        )> = Vec::new();
        match included_data.len() {
            0 => (),
            1 => {
                let el = included_data.into_iter().next().unwrap();
                if let Some(resource) = Self::insert_included_data_as_relationships(
                    base_type,
                    el,
                    main_data,
                    &mut late_linking,
                )? {
                    res.insert(resource.identifier().clone(), resource);
                }
            }
            _ => {
                for el in included_data.into_iter() {
                    if let Some(resource) = Self::insert_included_data_as_relationships(
                        base_type,
                        el,
                        main_data,
                        &mut late_linking,
                    )? {
                        res.insert(resource.identifier().clone(), resource);
                    }
                }
            }
        }
        Self::late_linking(&mut res, late_linking)?;
        Ok(res)
    }

    fn extract_main_data(
        main_data: IndexMap<
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
        base_type: &Arc<CibouletteResourceType>,
        el: CibouletteResponseElement<'response, B>,
        main_data: &mut IndexMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        late_linking_list: &mut Vec<(
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseElementAlias<'response>,
        )>,
    ) -> Result<Option<CibouletteResponseResource<'response, B>>, CibouletteError> {
        let main_id = el.identifier().clone();
        let main_type = el.type_().clone();

        let related = match el.related {
            Some(x) => x,
            None => {
                return Err(CibouletteError::UnknownError(
                    "No related data for orphaned row".into(),
                ))
            }
        };
        match related.rel_chain().len() {
            0 => {
                return Err(CibouletteError::UnknownError(
                    "No relationship chain for related element".into(),
                ))
            }
            1 => {
                if let Some(main_el) = main_data.get_mut(&main_id) {
                    insert_relationships_into_existing(
                        main_el,
                        related.element.clone(),
                        related.rel_chain().first().unwrap().relation_alias(),
                    );
                } else {
                    late_linking_list.push((main_id, related.clone()))
                }
            }
            _ => late_linking_list.push((main_id, related.clone())),
        }
        if &main_type == base_type && main_data.contains_key(related.element()) {
            Ok(None)
        } else {
            let resource = CibouletteResponseResource::<B> {
                type_: main_type,
                identifier: related.element,
                attributes: el.data,
                relationships: BTreeMap::default(),
                links: Option::default(),
            };
            Ok(Some(resource))
        }
    }

    fn late_linking(
        included_data: &mut BTreeMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        late_linking_list: Vec<(
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseElementAlias<'response>,
        )>,
    ) -> Result<(), CibouletteError> {
        for (k, v) in late_linking_list.into_iter() {
            let v_type = v.element().type_().clone();
            if let Some(el) = included_data.get_mut(&k) {
                insert_relationships_into_existing(
                    el,
                    v.element,
                    v.rel_chain
                        .last()
                        .ok_or_else(|| {
                            CibouletteError::UnknownError("Unbounded relationship".to_string())
                        })?
                        .relation_alias(),
                );
            } else {
                return Err(CibouletteError::MissingLink(
                    k.type_().to_string(),
                    v_type.to_string(),
                )); // FIXME Maybe not the most descriptive error
            }
        }
        Ok(())
    }
}

/// Inserts into an existing relationships a new entry, updating its format if necessary
fn insert_relationships_into_existing<'response, B>(
    obj: &mut CibouletteResponseResource<'response, B>,
    alias_identifier: CibouletteResourceResponseIdentifier<'response>,
    alias_str: &ArcStr,
) {
    if let Some(rel) = obj.relationships_mut().get_mut(alias_str) {
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
            alias_str.clone(),
            CibouletteResponseRelationshipObject {
                // TODO links
                data: CibouletteOptionalData::Object(
                    CibouletteResourceResponseIdentifierSelector::One(alias_identifier.clone()),
                ),
                ..Default::default()
            },
        );
    }
}
