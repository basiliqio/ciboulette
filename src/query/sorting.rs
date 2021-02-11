use super::*;

#[derive(Debug)]
pub enum CibouletteSortingDirection {
    Asc,
    Desc,
}

pub fn parse_sorting<'a>(s: &'a str) -> Vec<(CibouletteSortingDirection, Vec<&'a str>)> {
    let mut res: Vec<(CibouletteSortingDirection, Vec<&'a str>)> = Vec::new();

    for el in s.split(',') {
        if el.starts_with('-') {
            res.push((
                CibouletteSortingDirection::Desc,
                explode_by_comma(el.split_at(1).1),
            ))
        } else if el.starts_with('+') {
            res.push((
                CibouletteSortingDirection::Asc,
                explode_by_comma(el.split_at(1).1),
            ))
        } else {
            res.push((CibouletteSortingDirection::Asc, explode_by_comma(el)))
        }
    }
    res
}
