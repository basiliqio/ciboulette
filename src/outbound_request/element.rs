use super::*;
#[derive(Debug, Getters, Clone, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResponseElement<'a, B> {
    #[serde(skip_serializing)]
    pub(crate) type_: &'a CibouletteResourceType<'a>,
    pub(crate) identifier: CibouletteResourceIdentifier<'a>,
    pub(crate) data: Option<B>,
    pub(crate) related: Option<CibouletteResourceIdentifier<'a>>,
}

pub(super) fn fold_elements<'a, B, I>(
    ciboulette_store: &'a CibouletteStore<'a>,
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
                true => fold_elements_id(&ciboulette_store, inbound_request, &mut acc, x)?,
                false => fold_elements_obj(&ciboulette_store, inbound_request, &mut acc, x)?,
            },
            false => fold_elements_obj_other(&ciboulette_store, inbound_request, &mut acc, x),
        }
        if let Some(max) = acc.max_elements() {
            if acc.main_data().len() >= *max {
                todo!();
            }
        }
        Ok(acc)
    })
}

pub(super) fn fold_elements_id<'a, B>(
    ciboulette_store: &'a CibouletteStore<'a>,
    request: &dyn CibouletteInboundRequestCommons<'a>,
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
) -> Result<(), CibouletteError> {
    if element.data.is_some() {
        todo!();
    }
    let resource = CibouletteResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: None,
        relationships: BTreeMap::default(),
        links: Option::default(),
    };
    if matches!(
        acc.main_data_mut()
            .insert(resource.identifier().clone(), resource),
        Some(_)
    ) {
        todo!()
    }
    Ok(())
}

pub(super) fn fold_elements_obj<'a, B>(
    ciboulette_store: &'a CibouletteStore<'a>,
    request: &dyn CibouletteInboundRequestCommons<'a>,
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
) -> Result<(), CibouletteError> {
    let resource = CibouletteResource {
        type_: element.type_,
        identifier: element.identifier,
        attributes: element.data,
        relationships: BTreeMap::default(),
        links: Option::default(),
    };
    if matches!(
        acc.main_data_mut()
            .insert(resource.identifier().clone(), resource),
        Some(_)
    ) {
        todo!()
    }
    Ok(())
}

pub(super) fn fold_elements_obj_other<'a, B>(
    ciboulette_store: &'a CibouletteStore<'a>,
    request: &dyn CibouletteInboundRequestCommons<'a>,
    acc: &mut CibouletteOutboundRequestDataAccumulator<'a, B>,
    element: CibouletteResponseElement<'a, B>,
) {
    acc.included_data_mut().push(element);
}
