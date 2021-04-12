use serde_json::json;
use std::borrow::Cow;
use std::convert::TryFrom;
use url::Url;

use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
use insta::*;

mod build_failure;
mod delete_success;
mod insert_success;
mod select_success;
mod update_success;

fn gen_data_row<'request>(
    store: &CibouletteStore,
    type_: &'request str,
    id: &'request str,
    join_data: bool,
) -> CibouletteResponseElement<'request, String> {
    CibouletteResponseElement::new(
        &store,
        CibouletteResourceIdentifierBuilder::new(
            Some(CibouletteIdBuilder::Text(Cow::Borrowed(id))),
            Cow::Borrowed(type_),
        )
        .build_from_store(&store)
        .unwrap(),
        match join_data {
            true => Some(String::from("<some data>")),
            false => None,
        },
        None,
    )
    .unwrap()
}

fn gen_data_row_related<'request>(
    store: &CibouletteStore,
    type_: &'request str,
    id: &'request str,
    join_data: bool,
    related_type_: &'request str,
    related_id: &'request str,
) -> CibouletteResponseElement<'request, String> {
    CibouletteResponseElement::new(
        &store,
        CibouletteResourceIdentifierBuilder::new(
            Some(CibouletteIdBuilder::Text(Cow::Borrowed(id))),
            Cow::Borrowed(type_),
        )
        .build_from_store(&store)
        .unwrap(),
        match join_data {
            true => Some(String::from("<some data>")),
            false => None,
        },
        Some(
            CibouletteResourceIdentifierBuilder::new(
                Some(CibouletteIdBuilder::Text(Cow::Borrowed(related_id))),
                Cow::Borrowed(related_type_),
            )
            .build_from_store(&store)
            .unwrap(),
        ),
    )
    .unwrap()
}
