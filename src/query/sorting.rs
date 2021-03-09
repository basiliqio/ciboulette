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
