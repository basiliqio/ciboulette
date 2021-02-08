use super::*;

#[test]
fn duplicate_field_top_level() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "englisheeeeee",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		},
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "englisheeeeee",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn duplicate_field_data_level() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "englisheeeeee",
			"type": "englisheeeeee",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn wrong_argument_type_id() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": 65,
			"type": "englisheeeeee",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn wrong_argument_type_type() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": 3,
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn wrong_argument_data_types() {
    const VAL: &str = r#"
	{
		"data": 3
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn unknown_field() {
    const VAL: &str = r#"
	{
		"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "english",
			"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
}

#[test]
fn unknown_meta() {
    const META: &str = r#"
		{
			"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
			"como": 5317
		}
	"#;
    const VAL: &str = r#"
	{
		"meta":
		{
			"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
			"como": 5317
		},
		"data":
		{
			"meta":
			{
				"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
				"como": 5317
			},
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "english",
			"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
			"attributes":
			{
				"hello": "world",
				"world": "the earth"
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevelBuilder::deserialize(&mut deserializer).expect("no error");
    let meta_expected: serde_json::Value = serde_json::from_str(META).unwrap();
    assert_eq!(&meta_expected, doc.meta(), "meta object mismatch");
    match doc.data().as_ref().unwrap() {
        CibouletteResourceSelectorBuilder::One(data) => assert_eq!(
            &meta_expected,
            data.identifier().meta(),
            "meta object mismatch"
        ),
        _ => panic!(""),
    };
}
