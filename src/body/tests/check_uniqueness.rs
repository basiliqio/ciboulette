use super::*;

#[test]
fn uniq_obj() {
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
				}
			},
			{
				"id": "f15fa424-8aec-452f-a916-eb57c87bd172",
				"type": "comments",
				"attributes":
				{
					"body": "world2"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    doc.build(&bag).expect("no error");
}

#[test]
fn uniq_linked() {
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
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    doc.build(&bag).expect("no error");
}

#[test]
fn non_uniq_obj() {
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
				}
			},
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "comments",
				"attributes":
				{
					"body": "le monde"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqObj(type_, id) => {
            assert_eq!(type_, "comments".to_string(), "id mismatch");
            assert_eq!(
                id,
                "6720877a-e27e-4e9e-9ac0-3fff4deb55f2".to_string(),
                "id mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}

#[test]
fn non_uniq_rel() {
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
				},
				"articles":
				{
				  "links":
				  {
					"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/articles",
					"related": "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/comments"
				  },
				  "data":
				  {
					"type": "peoples",
					"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				  }
				}
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqRelationshipObject(type_, id) => {
            assert_eq!(type_, "peoples".to_string(), "id mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "id mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}

#[test]
fn non_uniq_rel2() {
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
				  [
					  {
						"type": "peoples",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  },
					  {
						"type": "peoples",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  }
				  ]
				}
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqRelationshipObject(type_, id) => {
            assert_eq!(type_, "peoples".to_string(), "id mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "id mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}

#[test]
fn uniq_rel() {
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
					  [
						  {
							"type": "peoples",
							"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
						  }
					  ]
					}
				}
			},
			{
				"id": "568109e1-74e9-41b3-a10f-f103aba5e78c",
				"type": "comments",
				"attributes":
				{
					"body": "world2"
				},
				"relationships":
				{
					"author":
					{
					  "links":
					  {
						"self": "/comments/568109e1-74e9-41b3-a10f-f103aba5e78c/relationships/author",
						"related": "/peoples/568109e1-74e9-41b3-a10f-f103aba5e78c/comments"
					  },
					  "data":
					  [
						  {
							"type": "peoples",
							"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
						  }
					  ]
					}
				}
			}
		],
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
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    doc.build(&bag).expect("no error");
}

#[test]
fn non_uniq_linked() {
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
			},
			"relationships":
			{
				"author":
				{
				  "links":
				  {
					"self": "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author",
					"related": "/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/comments"
				  },
				  "data":
				  {
					"type": "peoples",
					"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				  }
				}
			}
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
			},
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
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqObj(type_, id) => {
            assert_eq!(type_, "peoples".to_string(), "id mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "id mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}
