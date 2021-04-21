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
    let field = CibouletteQueryParametersBuilder::check_field_exists(&main_type, s.as_ref())?;
    Ok(CibouletteSortingElement { direction, field })
}
