use super::*;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum CibouletteSortingDirection {
    Asc,
    Desc,
}

pub fn parse_sorting<'a>(s: &str) -> Vec<(CibouletteSortingDirection, Cow<'a, str>)> {
    let mut res: Vec<(CibouletteSortingDirection, Cow<'a, str>)> = Vec::new();

    for el in s.split(',') {
        if el.starts_with('-') {
            res.push((
                CibouletteSortingDirection::Desc,
                Cow::Owned(el.borrow().split_at(1).1.to_string()),
            ))
        } else if el.starts_with('+') {
            res.push((
                CibouletteSortingDirection::Asc,
                Cow::Owned(el.borrow().split_at(1).1.to_string()),
            ))
        } else {
            res.push((
                CibouletteSortingDirection::Asc,
                Cow::Owned(el.borrow().to_string()),
            ))
        }
    }
    res
}
