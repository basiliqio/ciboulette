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
				"type": "comments",
				"attributes":
				{
					"body": "world"
				},
				"relationships":
				{
					"author":
					{
					  "links":
					  {
						"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author",
						"related": "/author/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/comments"
					  },
					  "data":
					  {
						"type": "peoples",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "peoples",
				"attributes":
				{
					"first-name": "john",
					"last-name": "doe"
				},
				"links":
				{
					"self": "/peoples/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    doc_builder
        .build(
            &bag,
            &CibouletteIntention::Read,
            bag.get_type("comments").unwrap(),
        )
        .expect("to build the document");
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
				"type": "comments",
				"attributes":
				{
					"body": "world"
				},
				"relationships":
				{
					"author":
					{
					  "links":
					  {
						"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author",
						"related": "/author/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/comments"
					  },
					  "data":
					  {
						"type": "peoples",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included": []
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err = doc_builder
        .build(
            &bag,
            &CibouletteIntention::Read,
            bag.get_type("comments").unwrap(),
        )
        .expect_err("missing link");
    match err {
        CibouletteError::MissingLink(type_, id) => {
            assert_eq!(type_, "peoples".to_string(), "type mismatch");
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
				"type": "comments",
				"attributes":
				{
					"body": "world"
				},
				"relationships":
				{
					"author":
					{
					  "links":
					  {
						"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author",
						"related": "/author/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/comments"
					  },
					  "data":
					  {
						"type": "peoples",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
					}
				}
			}
		],
		"links":
		{
			"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		},
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "peoples",
				"links":
				{
					"self": "/peoples/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let err = doc_builder
        .build(
            &bag,
            &CibouletteIntention::Read,
            bag.get_type("comments").unwrap(),
        )
        .expect_err("missing link");
    match err {
        CibouletteError::NoCompleteLinkage(type_, id) => {
            assert_eq!(type_, "peoples".to_string(), "type mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "type mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}
