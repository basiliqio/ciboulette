use super::*;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub enum CibouletteSortingDirection {
    Asc,
    Desc,
}

/// Parse the sorting argument, extracting the potential initial `-`/`+` operator
pub fn parse_sorting<'a>(s: &str) -> Vec<(CibouletteSortingDirection, Cow<'a, str>)> {
    let mut res: Vec<(CibouletteSortingDirection, Cow<'a, str>)> = Vec::new();

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
pub fn extract_type<'a>(
    bag: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    direction: CibouletteSortingDirection,
    s: Cow<'a, str>,
) -> Result<CibouletteSortingElement<'a>, CibouletteError> {
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
            CibouletteQueryParametersBuilder::check_field_exists(&main_type, field_name.as_ref())?;
            Ok(CibouletteSortingElement {
                type_: main_type,
                direction,
                field: Cow::Owned(field_name.into_owned()),
            })
        }
        1 => {
            list.insert(0, Cow::Borrowed(main_type.name().as_str()));
            let type_ = CibouletteQueryParametersBuilder::check_relationship_exists(&bag, &list)?;
            CibouletteQueryParametersBuilder::check_field_exists(&type_, field_name.as_ref())?;
            Ok(CibouletteSortingElement {
                type_,
                direction,
                field: Cow::Owned(field_name.into_owned()),
            })
        }
        _ => Err(CibouletteError::NestedSorting),
    }
}
