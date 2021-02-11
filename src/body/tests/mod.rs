mod bogus_deserialize;
mod check_linkage;
mod check_uniqueness;
mod key_clash;
mod normal_multi_docs;
mod normal_single_doc;

use super::*;
use crate::tests::*;

fn parse_attribute_comments() -> serde_json::Value {
    let s = r#"{
		"body": "world"
	}"#;
    serde_json::from_str(s).unwrap()
}
