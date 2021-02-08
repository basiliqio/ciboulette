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
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				}
			},
			{
				"id": "f15fa424-8aec-452f-a916-eb57c87bd172",
				"type": "english",
				"attributes":
				{
					"hello": "le monde",
					"world": "la terre"
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
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				}
			},
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "english",
				"attributes":
				{
					"hello": "le monde",
					"world": "la terre"
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
            assert_eq!(type_, "english".to_string(), "id mismatch");
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
				},
				"bonjour":
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
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqRelationship(type_, id) => {
            assert_eq!(type_, "planet".to_string(), "id mismatch");
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
				  [
					  {
						"type": "planet",
						"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
					  },
					  {
						"type": "planet",
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
        CibouletteError::UniqRelationship(type_, id) => {
            assert_eq!(type_, "planet".to_string(), "id mismatch");
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
					  [
						  {
							"type": "planet",
							"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
						  }
					  ]
					}
				}
			},
			{
				"id": "568109e1-74e9-41b3-a10f-f103aba5e78c",
				"type": "english",
				"attributes":
				{
					"hello": "world2",
					"world": "the earth v2"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/568109e1-74e9-41b3-a10f-f103aba5e78c/relationships/planet",
						"related": "/planet/568109e1-74e9-41b3-a10f-f103aba5e78c/english"
					  },
					  "data":
					  [
						  {
							"type": "planet",
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
			},
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
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let err: CibouletteError = doc.build(&bag).expect_err("uniqueness error");
    match err {
        CibouletteError::UniqObj(type_, id) => {
            assert_eq!(type_, "planet".to_string(), "id mismatch");
            assert_eq!(
                id,
                "b922a277-aadb-4c4e-b13d-9c4c98b3ad80".to_string(),
                "id mismatch"
            );
        }
        _ => panic!("wrong error type"),
    };
}
