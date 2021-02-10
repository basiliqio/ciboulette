use super::*;

mod parsing;
mod sorting;
mod sparse;
mod sparse_regex;

pub use sorting::CibouletteSortingDirection;

fn explode_by_comma<'a>(i: &'a str) -> Vec<Cow<'a, str>> {
    i.split(',').map(Cow::Borrowed).collect()
}
