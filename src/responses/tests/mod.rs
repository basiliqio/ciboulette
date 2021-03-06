pub use serde_json::json;
pub use std::borrow::Cow;
pub use std::convert::TryFrom;
pub use std::sync::Arc;
pub use url::Url;

use crate::test_helper::*;
use crate::*;

pub use insta::*;

mod build_failure;
mod delete_success;
mod insert_success;
mod select_success;
mod update_success;

pub fn gen_data_row<'request>(
    store: &CibouletteStore,
    base_type: &Arc<CibouletteResourceType>,
    type_: &'request str,
    id: &'request str,
    join_data: bool,
) -> CibouletteResponseElement<'request, String> {
    CibouletteResponseElement::build(
        &store,
        &base_type,
        CibouletteResourceIdentifierBuilder::new(Some(Cow::Borrowed(id)), Cow::Borrowed(type_)),
        match join_data {
            true => Some(String::from("<some data>")),
            false => None,
        },
        None,
    )
    .unwrap()
}

pub fn gen_data_row_related<'request>(
    store: &CibouletteStore,
    base_type: &Arc<CibouletteResourceType>,
    type_: &'request str,
    id: &'request str,
    join_data: bool,
    related_type_: &'request str,
    related_id: &'request str,
) -> CibouletteResponseElement<'request, String> {
    CibouletteResponseElement::build(
        &store,
        &base_type,
        CibouletteResourceIdentifierBuilder::new(Some(Cow::Borrowed(id)), Cow::Borrowed(type_)),
        match join_data {
            true => Some(String::from("<some data>")),
            false => None,
        },
        Some(CibouletteResourceIdentifierBuilder::new(
            Some(Cow::Borrowed(related_id)),
            Cow::Borrowed(related_type_),
        )),
    )
    .unwrap()
}
