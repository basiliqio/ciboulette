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
pub fn extract_type<'request>(
    bag: &CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    direction: CibouletteSortingDirection,
    s: Cow<'request, str>,
) -> Result<CibouletteSortingElement, CibouletteError> {
    if s.is_empty() {
        return Err(CibouletteError::UnknownField(
            main_type.name().to_string(),
            "<empty>".to_string(),
        ));
    }
    let mut list: Vec<Cow<'_, str>> = s.as_ref().split('.').map(Cow::Borrowed).collect();
    let field_name: Cow<'_, str> = list.pop().unwrap(); // There is a check that the `s` var isn't empty, the list should be at least populated with the whole string. It should NEVER fail
    match list.len() {
        0 => {
            let field = CibouletteQueryParametersBuilder::check_field_exists(
                &main_type,
                field_name.as_ref(),
            )?;
            Ok(CibouletteSortingElement {
                type_: main_type.clone(),
                direction,
                field,
            })
        }
        1 => {
            if list[0].as_ref() != main_type.name().as_str() {
                list.insert(0, Cow::Borrowed(&*main_type.name().as_str()));
            }
            let type_ = CibouletteQueryParametersBuilder::check_relationship_exists(&bag, &list)?;
            let field =
                CibouletteQueryParametersBuilder::check_field_exists(&type_, field_name.as_ref())?;
            Ok(CibouletteSortingElement {
                type_: type_.clone(),
                direction,
                field,
            })
        }
        _ => Err(CibouletteError::NestedSorting),
    }
}
