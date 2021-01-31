use super::*;

fn run_nested_test(
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
            match obj.get("hello").unwrap() {
                CibouletteResourceSchemaValue::Obj(obj) => {
                    assert_eq!(
                        obj.len(),
                        1,
                        "The root object should only contain a single key"
                    );
                    assert_eq!(
                        obj.contains_key("the"),
                        true,
                        "The hello key should be present"
                    );
                    assert_eq!(
                        obj.get("the").unwrap(),
                        &expected,
                        "The 'the' key should be '{:#?}'",
                        expected
                    );
                }
                _ => panic!("..."),
            };
        }
        _ => panic!("..."),
    };
}

#[test]
fn nested_simple() {
    let nested_string =
        CibouletteResourceSchema::String(CibouletteResourceSchemaScalar::new(false));
    let nested_schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("the".to_string(), &nested_string)]
                .into_iter()
                .collect(),
            false,
        ));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), &nested_schema)]
                .into_iter()
                .collect(),
            false,
        ));
    let value = r#"
	{
		"hello": {
			"the": "world"
		}
	}
	"#;

    run_nested_test(
        schema,
        value,
        CibouletteResourceSchemaValue::String(Cow::Borrowed("world")),
    );
}

#[test]
fn nested_wrong_key() {
    let nested_string =
        CibouletteResourceSchema::String(CibouletteResourceSchemaScalar::new(false));
    let nested_schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("the".to_string(), &nested_string)]
                .into_iter()
                .collect(),
            false,
        ));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), &nested_schema)]
                .into_iter()
                .collect(),
            false,
        ));
    let value = r#"
	{
		"hello": {
			"hola": "world"
		}
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .deserialize(&mut deserializer)
        .expect_err("to fail because of wrong key");
}

#[test]
fn nested_wrong_value_type() {
    let nested_string =
        CibouletteResourceSchema::String(CibouletteResourceSchemaScalar::new(false));
    let nested_schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("the".to_string(), &nested_string)]
                .into_iter()
                .collect(),
            false,
        ));
    let schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Obj(CibouletteResourceSchemaObject::new(
            vec![("hello".to_string(), &nested_schema)]
                .into_iter()
                .collect(),
            false,
        ));
    let value = r#"
	{
		"hello": {
			"the": 61
		}
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .deserialize(&mut deserializer)
        .expect_err("to fail because of wrong value");
}
