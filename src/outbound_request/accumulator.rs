use super::*;

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(super) struct CibouletteOutboundRequestDataAccumulator<'a, B> {
    pub(super) main_data: BTreeMap<
        CibouletteResourceIdentifier<'a>,
        CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>,
    >,
    pub(super) included_data: Vec<CibouletteResponseElement<'a, B>>,
    max_elements: Option<usize>,
    only_ids: bool,
}

pub(super) struct CibouletteOutboundRequestExtractedData<'a, B> {
    pub main_data:
        CibouletteOptionalData<CibouletteResourceSelector<'a, B, CibouletteResourceIdentifier<'a>>>,
    pub included_data: Vec<CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>>,
}

impl<'a, B> CibouletteOutboundRequestDataAccumulator<'a, B> {
    pub fn new(max_elements: Option<usize>, only_ids: bool) -> Self {
        CibouletteOutboundRequestDataAccumulator {
            main_data: BTreeMap::default(),
            included_data: Vec::default(),
            max_elements,
            only_ids,
        }
    }

    pub fn init_from_request(
        inbound_request: &dyn CibouletteInboundRequestCommons<'a>,
    ) -> CibouletteOutboundRequestDataAccumulator<'a, B> {
        let acc: CibouletteOutboundRequestDataAccumulator<'a, B> =
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

    pub fn extract(
        self,
        inbound_request: &dyn CibouletteInboundRequestCommons<'a>,
    ) -> Result<CibouletteOutboundRequestExtractedData<'a, B>, CibouletteError> {
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
            CibouletteResourceIdentifier<'a>,
            CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>,
        >,
        included_data: Vec<CibouletteResponseElement<'a, B>>,
    ) -> Result<Vec<CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>>, CibouletteError>
    {
        match included_data.len() {
            0 => Ok(vec![]),
            1 => {
                let el = included_data.into_iter().next().unwrap();
                let resource = Self::insert_included_data_as_relationships(el, main_data)?;
                Ok(vec![resource])
            }
            _ => {
                let mut res: Vec<CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>> =
                    Vec::with_capacity(included_data.len());
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
            CibouletteResourceIdentifier<'a>,
            CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>,
        >,
        inbound_request: &dyn CibouletteInboundRequestCommons,
    ) -> CibouletteOptionalData<CibouletteResourceSelector<'a, B, CibouletteResourceIdentifier<'a>>>
    {
        let body_data: CibouletteBodyData<'a, CibouletteResourceIdentifier<'a>, B> =
            match inbound_request.expected_response_type() {
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
        el: CibouletteResponseElement<'a, B>,
        mut main_data: &mut BTreeMap<
            CibouletteResourceIdentifier<'a>,
            CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>,
        >,
    ) -> Result<CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>, CibouletteError> {
        let related = match el.related() {
            Some(x) => x,
            None => todo!(),
        };
        match main_data.get_mut(&related) {
            Some(main_el) => {
                insert_relationships_into_existing(main_el, &related, el.identifier().clone())?;
            }
            None => todo!(),
        };
        let resource = CibouletteResource::<B, CibouletteResourceIdentifier<'a>> {
            type_: el.type_,
            identifier: el.identifier,
            attributes: None,
            relationships: BTreeMap::default(),
            links: Option::default(),
        };
        Ok(resource)
    }
}

fn insert_relationships_into_existing<'a, B>(
    obj: &mut CibouletteResource<'a, B, CibouletteResourceIdentifier>,
    related: &CibouletteResourceIdentifier<'a>,
    identifier: CibouletteResourceIdentifier<'a>,
) -> Result<(), CibouletteError> {
    let alias = obj.type_().get_alias(related.type_())?;
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
            Cow::Borrowed(alias.as_str()),
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
