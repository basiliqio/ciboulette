use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;

mod filter;
mod filter_simple;
mod include;
mod meta;
mod page;
mod sorting;
mod sparse;

#[inline]
fn setup(input: &str) -> (CibouletteStore, CibouletteQueryParametersBuilder<'_>) {
    let bag = gen_bag();
    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(input).expect("to parse");

    (bag, builder)
}
