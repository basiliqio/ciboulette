use super::*;
/// Container for response element. While building a response, every object should be wrapped in this container
#[derive(Debug, Getters, Clone, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResponseElement<'request, 'store, B> {
    #[serde(skip_serializing)]
    /// The type of the contained value
    pub(crate) type_: Arc<CibouletteResourceType<'store>>,
    /// The identifier of the contained value
    pub(crate) identifier: CibouletteResourceIdentifier<'request>,
    /// The data of the contained value
    pub(crate) data: Option<B>,
    /// Some other identifier it relates to
    pub(crate) related: Option<CibouletteResourceIdentifier<'request>>,
}

impl<'request, 'store, B> CibouletteResponseElement<'request, 'store, B> {
    pub fn new(
        store: &'store CibouletteStore<'store>,
        identifier: CibouletteResourceIdentifier<'request>,
        data: Option<B>,
        related: Option<CibouletteResourceIdentifier<'request>>,
    ) -> Result<Self, CibouletteError> {
        let type_: Arc<CibouletteResourceType<'store>> =
            store.get_type(identifier.type_().as_ref())?.clone();
        Ok(CibouletteResponseElement {
            type_,
            identifier,
            data,
            related,
        })
    }
}

/// Fold elements into an accumulator for easier processing
pub(super) fn fold_elements<'request, 'store, B, I>(
    elements: I,
    acc: CibouletteOutboundRequestDataAccumulator<'request, 'store, B>,
    inbound_request: &dyn CibouletteInboundRequestCommons<'request, 'store>,
) -> Result<CibouletteOutboundRequestDataAccumulator<'request, 'store, B>, CibouletteError>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'request, 'store, B>>,
{
    elements.into_iter().try_fold(acc, |mut acc, x| {
        match x.identifier().type_() == inbound_request.path().main_type().name().as_str() {
            true => match acc.only_ids() {
                true => fold_elements_id(&mut acc, x),
                false => fold_elements_obj(&mut acc, x),
            },
            false => fold_elements_obj_other(&mut acc, x),
        }
        if let Some(max) = acc.max_elements() {
            if acc.main_data().len() > *max {
                return Err(CibouletteError::OutboundTooManyMainData(
                    inbound_request.path().main_type().name().to_string(),
                ));
            }
        }
        Ok(acc)
    })
}

pub(super) fn fold_elements_id<'request, 'store, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, 'store, B>,
    element: CibouletteResponseElement<'request, 'store, B>,
) {
    let resource = CibouletteResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: None,
        relationships: BTreeMap::default(),
        links: Option::default(),
        meta: None, //FIXME
    };
    acc.main_data_mut()
        .insert(resource.identifier().clone(), resource);
}

pub(super) fn fold_elements_obj<'request, 'store, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, 'store, B>,
    element: CibouletteResponseElement<'request, 'store, B>,
) {
    let resource = CibouletteResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: element.data,
        relationships: BTreeMap::default(),
        links: Option::default(),
        meta: None, //FIXME
    };
    acc.main_data_mut()
        .insert(resource.identifier().clone(), resource);
}

pub(super) fn fold_elements_obj_other<'request, 'store, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'request, 'store, B>,
    element: CibouletteResponseElement<'request, 'store, B>,
) {
    acc.included_data_mut().push(element);
}
