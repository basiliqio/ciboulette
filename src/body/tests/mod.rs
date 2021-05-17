mod bogus_deserialize;
mod check_linkage;
mod check_uniqueness;
mod key_clash;
mod normal_multi_docs;
mod normal_single_doc;
mod parsing_id;
mod request_data;
mod top_level_quick_checks;

use crate::test_helper::*;
use crate::*;
use serde::Deserialize;

fn parse_attribute_comments() -> serde_json::Value {
    let s = r#"{
		"body": "world"
	}"#;
    serde_json::from_str(s).unwrap()
}
