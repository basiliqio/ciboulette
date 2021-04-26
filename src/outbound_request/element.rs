use super::*;
/// Container for response element. While building a response, every object should be wrapped in this container
#[derive(Debug, Getters, Clone, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResponseElement<'request, B> {
    #[serde(skip_serializing)]
    /// The type of the contained value
    pub(crate) type_: Arc<CibouletteResourceType>,
    /// The identifier of the contained value
    pub(crate) identifier: CibouletteResourceResponseIdentifier<'request>,
    /// The data of the contained value
    pub(crate) data: Option<B>,
    /// Some other identifier it relates to
    pub(crate) related: Option<CibouletteResponseElementAlias<'request>>,
}

#[derive(Debug, Getters, Clone, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResponseElementAlias<'request> {
    #[serde(skip_serializing)]
    pub(crate) rel_chain: Vec<CibouletteResourceRelationshipDetails>,
    #[serde(flatten)]
    pub(crate) element: CibouletteResourceResponseIdentifier<'request>,
}
impl<'request> CibouletteResponseElementAlias<'request> {
    pub fn new(
        rel_chain: Vec<CibouletteResourceRelationshipDetails>,
        element: CibouletteResourceResponseIdentifier<'request>,
    ) -> Self {
        CibouletteResponseElementAlias { rel_chain, element }
    }
}

impl<'request, B> CibouletteResponseElement<'request, B> {
    pub fn new(
        store: &CibouletteStore,
        main_type: &Arc<CibouletteResourceType>,
        identifier: CibouletteResourceIdentifierBuilder<'request>,
        data: Option<B>,
        related: Option<CibouletteResourceIdentifierBuilder<'request>>,
    ) -> Result<Self, CibouletteError> {
        let (identifier, related) = match related {
            Some(related) => {
                let related_chain = CibouletteResourceResponseIdentifierBuilder::from(identifier)
                    .build_relationships(store, main_type.clone())?;
                (
                    CibouletteResourceResponseIdentifierBuilder::from(related).build(store)?,
                    Some(related_chain),
                )
            }
            None => {
                let identifier =
                    CibouletteResourceResponseIdentifierBuilder::from(identifier).build(store)?;
                (identifier, None)
            }
        };
        let type_ = match &related {
            Some(related) => store.get_type(related.element().type_())?,
            None => store.get_type(identifier.type_())?,
        }
        .clone();
        Ok(CibouletteResponseElement {
            type_,
            identifier,
            data,
            related,
        })
    }
}

/// Fold elements into an accumulator for easier processing
pub(super) fn fold_elements<'request, 'response, B, I>(
    elements: I,
    acc_settings: CibouletteOutboundRequestDataAccumulatorSettings,
) -> Result<CibouletteOutboundRequestDataAccumulator<'response, B>, CibouletteError>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'response, B>>,
{
    let acc = CibouletteOutboundRequestDataAccumulator::from(acc_settings.clone());
    elements.into_iter().try_fold(acc, |mut acc, x| {
        match x.related().is_none()
            && x.identifier().type_() == acc_settings.main_type().name().as_str()
        {
            true => match acc.settings().only_ids() {
                true => fold_elements_id(&mut acc, x),
                false => fold_elements_obj(&mut acc, x),
            },
            false => fold_elements_obj_other(&mut acc, x),
        }
        if let Some(max) = acc.settings().max_elements() {
            if acc.main_data().len() > *max {
                return Err(CibouletteError::OutboundTooManyMainData(
                    acc_settings.main_type().name().to_string(),
                ));
            }
        }
        Ok(acc)
    })
}

pub(super) fn fold_elements_id<'request, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, B>,
    element: CibouletteResponseElement<'request, B>,
) {
    let resource = CibouletteResponseResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: None,
        relationships: BTreeMap::default(),
        links: Option::default(),
    };
    acc.main_data_mut()
        .insert(resource.identifier().clone(), resource);
}

pub(super) fn fold_elements_obj<'request, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, B>,
    element: CibouletteResponseElement<'request, B>,
) {
    let resource = CibouletteResponseResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: element.data,
        relationships: BTreeMap::default(),
        links: Option::default(),
    };
    acc.main_data_mut()
        .insert(resource.identifier().clone(), resource);
}

pub(super) fn fold_elements_obj_other<'request, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, B>,
    element: CibouletteResponseElement<'request, B>,
) {
    acc.included_data_mut().push(element);
}
