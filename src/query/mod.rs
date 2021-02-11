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
use parsing::CibouletteQueryParametersBuilder;
use visitor::CibouletteQueryParametersBuilderVisitor;

pub use sorting::CibouletteSortingDirection;

fn explode_by_comma<'a>(i: &'a str) -> Vec<&'a str> {
    i.split(',').collect()
}
