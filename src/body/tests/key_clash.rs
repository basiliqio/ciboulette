use super::*;

#[test]
fn key_clash_data_error() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		},
		"errors":
		{
			"id": "toto",
			"status": 400
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteTopLevelBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err: CibouletteError = doc_builder.build(&bag).expect_err("keyclash error");
    match err {
        CibouletteError::KeyClash(key1, op, key2) => {
            assert_eq!(key1.as_str(), "data", "key mismatch");
            assert_eq!(key2.as_str(), "errors", "key mismatch");
            assert_eq!(
                matches!(op, CibouletteClashDirection::Without),
                true,
                "wrong operator"
            );
        }
        _ => panic!("wrong error type"),
    };
}

#[test]
fn key_clash_included_without_data() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "comments",
				"attributes":
				{
					"body": "world"
				},
				"links":
				{
					"self": "/comments/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		],
		"meta": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteTopLevelBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err: CibouletteError = doc_builder.build(&bag).expect_err("keyclash error");
    match err {
        CibouletteError::KeyClash(key1, op, key2) => {
            assert_eq!(key1.as_str(), "included", "key mismatch");
            assert_eq!(key2.as_str(), "data", "key mismatch");
            assert_eq!(
                matches!(op, CibouletteClashDirection::With),
                true,
                "wrong operator"
            );
        }
        _ => panic!("wrong error type"),
    };
}
