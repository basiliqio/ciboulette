use super::*;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub enum CibouletteSortingDirection {
    Asc,
    Desc,
}

/// Parse the sorting argument, extracting the potential initial `-`/`+` operator
pub fn parse_sorting<'request>(s: &str) -> Vec<(CibouletteSortingDirection, Cow<'request, str>)> {
    let mut res: Vec<(CibouletteSortingDirection, Cow<'request, str>)> = Vec::new();

    for el in s.split(',') {
        // Split by ','
        if el.starts_with('-') {
            // Descending
            res.push((
                CibouletteSortingDirection::Desc,
                Cow::Owned(el.borrow().split_at(1).1.to_string()),
            ))
        } else if el.starts_with('+') {
            // Ascending
            res.push((
                CibouletteSortingDirection::Asc,
                Cow::Owned(el.borrow().split_at(1).1.to_string()),
            ))
        } else {
            // By default, ascending
            res.push((
                CibouletteSortingDirection::Asc,
                Cow::Owned(el.borrow().to_string()),
            ))
        }
    }
    res
}

/// Parse the sorting argument, extracting the potential initial `-`/`+` operator
pub fn extract_type(
    store: &CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    direction: CibouletteSortingDirection,
    s: Cow<'_, str>,
) -> Result<CibouletteSortingElement, CibouletteError> {
    if s.is_empty() {
        return Err(CibouletteError::UnknownField(
            main_type.name().to_string(),
            "<empty>".to_string(),
        ));
    }
    let mut el_list: Vec<Cow<'_, str>> = s.split('.').map(Cow::Borrowed).collect();
    let field_raw = el_list.pop().unwrap_or_default();

    let rel_chain: Vec<CibouletteResourceRelationshipDetails> = match el_list.is_empty() {
        true => Vec::new(),
        false => CibouletteQueryParametersBuilder::check_relationship_exists(
            store,
            &main_type,
            el_list.as_slice(),
        )?,
    };
    let field = CibouletteQueryParametersBuilder::check_field_exists(
        &rel_chain
            .last()
            .map(|x| x.related_type())
            .unwrap_or(&main_type),
        field_raw.as_ref(),
    )?;

    Ok(CibouletteSortingElement {
        rel_chain,
        direction,
        field,
    })
}
