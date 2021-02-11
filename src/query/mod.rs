use super::*;

mod fields;
mod parsing;
mod sorting;
mod typed_param;
mod typed_param_regex;
mod visitor;

#[cfg(test)]
mod tests;

pub use fields::CiboulettePageType;
use fields::CibouletteQueryParametersField;
pub use parsing::{
    CibouletteQueryParameters, CibouletteQueryParametersBuilder, CibouletteSortingElement,
};
use visitor::CibouletteQueryParametersBuilderVisitor;

pub use sorting::CibouletteSortingDirection;

fn explode_by_comma(i: Cow<'_, str>) -> Vec<Cow<'_, str>> {
    i.split(',').map(str::to_string).map(Cow::Owned).collect()
}

fn explode_by_comma_and_dot(i: Cow<'_, str>) -> Vec<Vec<Cow<'_, str>>> {
    i.split(',')
        .map(|x| x.split('.').map(str::to_string).map(Cow::Owned).collect())
        .collect()
}
