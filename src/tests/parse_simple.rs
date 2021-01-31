use super::*;

fn run_flat_test(
    schema: CibouletteResourceSchema,
    value: &str,
    expected: CibouletteResourceSchemaValue,
) {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: CibouletteResourceSchemaValue = schema.deserialize(&mut deserializer).unwrap();
    assert_eq!(
        matches!(parsed, CibouletteResourceSchemaValue::Obj(_)),
        true,
        "The root should be an object"
    );
    match parsed {
        CibouletteResourceSchemaValue::Obj(obj) => {
            assert_eq!(
                obj.len(),
                1,
                "The root object should only contain a single key"
            );
            assert_eq!(
                obj.contains_key("hello"),
                true,
                "The hello key should be present"
            );
            assert_eq!(
                obj.get("hello").unwrap(),
                &expected,
                "The hello key should be '{:#?}'",
                expected
            );
        }
        _ => panic!("..."),
    };
}

#[test]
fn string() {
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![("hello".to_string(), &CibouletteResourceSchema::String)]
            .into_iter()
            .collect(),
    );
    let value = r#"
	{
		"hello": "world"
	}
	"#;

    run_flat_test(
        schema,
        value,
        CibouletteResourceSchemaValue::String(Cow::Borrowed("world")),
    );
}

#[test]
fn bool() {
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![("hello".to_string(), &CibouletteResourceSchema::Bool)]
            .into_iter()
            .collect(),
    );
    let value = r#"
	{
		"hello": true
	}
	"#;
    run_flat_test(schema, value, CibouletteResourceSchemaValue::Bool(true));
}

#[test]
fn number_tiny() {
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![(
            "hello".to_string(),
            &CibouletteResourceSchema::Number(CibouletteResourceSchemaNumberType::U64),
        )]
        .into_iter()
        .collect(),
    );
    let value = r#"
	{
		"hello": 15
	}
	"#;
    run_flat_test(schema, value, CibouletteResourceSchemaValue::Number(15));
}

#[test]
fn number_huge() {
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![(
            "hello".to_string(),
            &CibouletteResourceSchema::Number(CibouletteResourceSchemaNumberType::U128),
        )]
        .into_iter()
        .collect(),
    );
    let value = r#"
	{
		"hello": 340282366920938463463374607431768211454
	}
	"#;
    run_flat_test(
        schema,
        value,
        CibouletteResourceSchemaValue::Number(340282366920938463463374607431768211454),
    );
}
