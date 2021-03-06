use super::*;
use element::CibouletteResponseElementAlias;

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteResponseDataAccumulator<'response, B> {
    pub(super) main_data: indexmap::IndexMap<
        CibouletteResourceResponseIdentifier<'response>,
        CibouletteResponseResource<'response, B>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'response, B>>,
    settings: CibouletteResponseDataAccumulatorSettings,
}

/// Hold data while building the outbound response
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteResponseDataAccumulatorSettings {
    max_elements: Option<usize>,
    only_ids: bool,
    main_type: Arc<CibouletteResourceType>,
    include_rels: Option<CibouletteResourceRelationshipDetails>,
}

impl CibouletteResponseDataAccumulatorSettings {
    pub fn new(
        main_type: Arc<CibouletteResourceType>,
        max_elements: Option<usize>,
        only_ids: bool,
        include_rels: Option<CibouletteResourceRelationshipDetails>,
    ) -> Self {
        CibouletteResponseDataAccumulatorSettings {
            max_elements,
            only_ids,
            main_type,
            include_rels,
        }
    }
}

impl<'request> From<&dyn CibouletteRequestCommons<'request>>
    for CibouletteResponseDataAccumulatorSettings
{
    fn from(inbound_request: &dyn CibouletteRequestCommons<'request>) -> Self {
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
        CibouletteResponseDataAccumulatorSettings::new(
            inbound_request.expected_type().clone(),
            max_element,
            only_ids,
            include_rels,
        )
    }
}

impl<'response, B> From<CibouletteResponseDataAccumulatorSettings>
    for CibouletteResponseDataAccumulator<'response, B>
{
    fn from(settings: CibouletteResponseDataAccumulatorSettings) -> Self {
        CibouletteResponseDataAccumulator {
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

impl<'response, B> CibouletteResponseDataAccumulator<'response, B> {
    /// Extract the accumulated data
    pub fn extract<'request, 'store>(
        self,
        config: &'store CibouletteConfig,
        inbound_request: &dyn CibouletteRequestCommons<'request>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'response, B>, CibouletteError> {
        let settings = self.settings;
        let mut main_data = self.main_data;
        match settings.include_rels() {
            None => {
                let included_data = Self::extract_included_data(
                    config,
                    inbound_request.expected_type(),
                    &mut main_data,
                    self.included_data,
                )?;
                let body_data = Self::extract_main_data(config, main_data, inbound_request);
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
                let body_data = Self::extract_main_data(config, main_data, inbound_request);
                Ok(CibouletteOutboundRequestExtractedData {
                    main_data: body_data,
                    included_data: BTreeMap::default(),
                })
            }
        }
    }

    fn extract_included_data<'store>(
        config: &'store CibouletteConfig,
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
                    config,
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
                        config,
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
        Self::late_linking(config, &mut res, late_linking)?;
        Ok(res)
    }

    fn extract_main_data<'store>(
        config: &'store CibouletteConfig,
        main_data: IndexMap<
            CibouletteResourceResponseIdentifier<'response>,
            CibouletteResponseResource<'response, B>,
        >,
        inbound_request: &dyn CibouletteRequestCommons,
    ) -> CibouletteOptionalData<CibouletteResponseResourceSelector<'response, B>> {
        let body_data: CibouletteResponseBodyData<'response, B> =
            match inbound_request.expected_response_type() {
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single)
                | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Single) => {
                    match main_data.into_iter().next() {
                        Some((_, mut response_resource)) => {
                            *response_resource.links_mut() =
                                super::links::build_link_for_response_object(
                                    config,
                                    response_resource.identifier(),
                                );
                            CibouletteOptionalData::Object(CibouletteResponseResourceSelector::One(
                                response_resource,
                            ))
                        }
                        None => CibouletteOptionalData::Null(true),
                    }
                }
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple)
                | CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple) => {
                    let mut res = Vec::with_capacity(main_data.len());
                    for (_, mut el) in main_data.into_iter() {
                        *el.links_mut() =
                            super::links::build_link_for_response_object(config, el.identifier());
                        res.push(el);
                    }
                    CibouletteOptionalData::Object(CibouletteResponseResourceSelector::Many(res))
                }
                CibouletteResponseRequiredType::None => CibouletteOptionalData::Null(true),
            };
        body_data
    }

    fn insert_included_data_as_relationships<'store>(
        config: &'store CibouletteConfig,
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
                        config,
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
                links: super::links::build_link_for_response_object(config, &related.element),
                identifier: related.element,
                attributes: el.data,
                relationships: BTreeMap::default(),
            };
            Ok(Some(resource))
        }
    }

    fn late_linking<'store>(
        config: &'store CibouletteConfig,
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
                    config,
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
fn insert_relationships_into_existing<'store, 'response, B>(
    config: &'store CibouletteConfig,
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
        let links =
            super::links::build_link_for_response_relationship(config, obj.identifier(), alias_str);
        obj.relationships_mut().insert(
            alias_str.clone(),
            CibouletteResponseRelationshipObject {
                links,
                data: CibouletteOptionalData::Object(
                    CibouletteResourceResponseIdentifierSelector::One(alias_identifier.clone()),
                ),
            },
        );
    }
}
