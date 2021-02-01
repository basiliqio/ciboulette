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
    let nested_string =
        CibouletteResourceSchema::String(CibouletteResourceSchemaScalar::new(false));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(Box::new(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), nested_string)]
                .into_iter()
                .collect(),
            false,
        )));
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
    let nested_string = CibouletteResourceSchema::Bool(CibouletteResourceSchemaScalar::new(false));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(Box::new(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), nested_string)]
                .into_iter()
                .collect(),
            false,
        )));
    let value = r#"
	{
		"hello": true
	}
	"#;
    run_flat_test(schema, value, CibouletteResourceSchemaValue::Bool(true));
}

#[test]
fn number_tiny() {
    let nested_string = CibouletteResourceSchema::Number(CibouletteResourceSchemaNumeric::new(
        CibouletteResourceSchemaNumberType::U64,
        false,
    ));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(Box::new(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), nested_string)]
                .into_iter()
                .collect(),
            false,
        )));
    let value = r#"
	{
		"hello": 15
	}
	"#;
    run_flat_test(schema, value, CibouletteResourceSchemaValue::Number(15));
}

#[test]
fn number_huge() {
    let nested_string = CibouletteResourceSchema::Number(CibouletteResourceSchemaNumeric::new(
        CibouletteResourceSchemaNumberType::U128,
        false,
    ));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(Box::new(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), nested_string)]
                .into_iter()
                .collect(),
            false,
        )));
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
