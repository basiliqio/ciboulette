use super::*;

#[test]
fn ok() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/planet",
						"related": "/planet/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/english"
					  },
					  "data":
					  {
						"type": "planet",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "planet",
				"attributes":
				{
					"p": "earth"
				},
				"links":
				{
					"self": "/planet/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteTopLevelBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    doc_builder.build(&bag).expect("to build the document");
}

#[test]
fn missing_link() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/planet",
						"related": "/planet/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/english"
					  },
					  "data":
					  {
						"type": "planet",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included": []
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteTopLevelBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err = doc_builder.build(&bag).expect_err("missing link");
    match err {
        CibouletteError::MissingLink(type_, id) => {
            assert_eq!(type_, "planet".to_string(), "type mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "type mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}

#[test]
fn not_fully_linked() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/planet",
						"related": "/planet/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/english"
					  },
					  "data":
					  {
						"type": "planet",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "planet",
				"links":
				{
					"self": "/planet/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteTopLevelBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err = doc_builder.build(&bag).expect_err("missing link");
    match err {
        CibouletteError::NoCompleteLinkage(type_, id) => {
            assert_eq!(type_, "planet".to_string(), "type mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "type mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}
