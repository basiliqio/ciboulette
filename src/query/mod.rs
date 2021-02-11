use super::*;

mod fields;
mod parsing;
mod sorting;
mod typed_param;
mod typed_param_regex;
mod visitor;

#[cfg(test)]
mod tests;

use fields::{CiboulettePageType, CibouletteQueryParametersField};
pub use parsing::{
    CibouletteQueryParameters, CibouletteQueryParametersBuilder, CibouletteSortingElement,
};
use visitor::CibouletteQueryParametersBuilderVisitor;

pub use sorting::CibouletteSortingDirection;

fn explode_by_comma<'a>(i: Cow<'a, str>) -> Vec<Cow<'a, str>> {
    i.split(',').map(str::to_string).map(Cow::Owned).collect()
}

fn explode_by_dot<'a>(i: Cow<'a, str>) -> Vec<Cow<'a, str>> {
    i.split('.').map(str::to_string).map(Cow::Owned).collect()
}
