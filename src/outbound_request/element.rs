use super::*;
/// Container for response element. While building a response, every object should be wrapped in this container
#[derive(Debug, Getters, Clone, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResponseElement<'a, B> {
    #[serde(skip_serializing)]
    /// The type of the contained value
    pub(crate) type_: &'a CibouletteResourceType<'a>,
    /// The identifier of the contained value
    pub(crate) identifier: CibouletteResourceIdentifier<'a>,
    /// The data of the contained value
    pub(crate) data: Option<B>,
    /// Some other identifier it relates to
    pub(crate) related: Option<CibouletteResourceIdentifier<'a>>,
}

impl<'a, B> CibouletteResponseElement<'a, B> {
    pub fn new(
        store: &'a CibouletteStore<'a>,
        identifier: CibouletteResourceIdentifier<'a>,
        data: Option<B>,
        related: Option<CibouletteResourceIdentifier<'a>>,
    ) -> Result<Self, CibouletteError> {
        Ok(CibouletteResponseElement {
            type_: store.get_type(identifier.type_().as_ref())?,
            identifier,
            data,
            related,
        })
    }
}

/// Fold elements into an accumulator for easier processing
pub(super) fn fold_elements<'a, B, I>(
    elements: I,
    acc: CibouletteOutboundRequestDataAccumulator<'a, B>,
    inbound_request: &dyn CibouletteInboundRequestCommons<'a>,
) -> Result<CibouletteOutboundRequestDataAccumulator<'a, B>, CibouletteError>
where
    B: Serialize,
    I: IntoIterator<Item = CibouletteResponseElement<'a, B>>,
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
                    inbound_request.path().main_type().name().clone(),
                ));
            }
        }
        Ok(acc)
    })
}

pub(super) fn fold_elements_id<'a, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
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

pub(super) fn fold_elements_obj<'a, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
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

pub(super) fn fold_elements_obj_other<'a, B>(
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
) {
    acc.included_data_mut().push(element);
}
