use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
use serde::Deserialize;

mod include;
mod sorting;
mod sparse;

#[inline]
fn setup(input: &str) -> (CibouletteBag, CibouletteQueryParametersBuilder<'_>) {
    let bag = gen_bag();
    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(input).expect("to parse");

    (bag, builder)
}
