use super::*;

#[test]
fn duplicate_field_top_level() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "commentsssss",
			"attributes":
			{
				"body": "hello"
			}
		},
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "commentsssss",
			"attributes":
			{
				"body": "hello"
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn duplicate_field_data_level() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "comments",
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn wrong_argument_type_id() {
    const VAL: &str = r#"
	{
		"data":
		{
			"id": 65,
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect_err("an error");
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
				"body": "world"
			}
		},
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn wrong_argument_data_types() {
    const VAL: &str = r#"
	{
		"data": 3
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect_err("an error");
}

#[test]
fn unknown_field() {
    const VAL: &str = r#"
	{
		"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "comments",
			"HOLAAAAAAAAAAAAAAA": "CHICOOOOOOOOOOOOOOOOOOOOOOOOOOOOOS",
			"attributes":
			{
				"body": "world"
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    CibouletteBodyBuilder::deserialize(&mut deserializer).expect("no error");
}
