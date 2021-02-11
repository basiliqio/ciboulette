mod bogus_deserialize;
mod check_linkage;
mod check_uniqueness;
mod key_clash;
mod normal_multi_docs;
mod normal_single_doc;

use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
use serde::Deserialize;

fn parse_attribute_comments() -> serde_json::Value {
    let s = r#"{
		"body": "world"
	}"#;
    serde_json::from_str(s).unwrap()
}
