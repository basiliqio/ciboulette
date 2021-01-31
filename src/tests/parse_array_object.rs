use super::*;

fn run_nested_test(schema: CibouletteResourceSchema, value: &str) {
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
                CibouletteResourceSchemaValue::Array(arr) => {
                    assert_eq!(arr.len(), 2, "The hello array should contain a 2 keys");
                    assert_eq!(
                        arr[0],
                        CibouletteResourceSchemaValue::String(Cow::Borrowed("the")),
                        "Value mismatch"
                    );
                    assert_eq!(
                        arr[1],
                        CibouletteResourceSchemaValue::String(Cow::Borrowed("world")),
                        "Value mismatch"
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
    let nested_schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Array(&CibouletteResourceSchema::String);
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![("hello".to_string(), &nested_schema)]
            .into_iter()
            .collect(),
    );
    let value = r#"
	{
		"hello": [
			"the",
			"world"
		]
	}
	"#;

    run_nested_test(schema, value);
}

#[test]
fn nested_wrong_value() {
    let nested_schema: CibouletteResourceSchema =
        CibouletteResourceSchema::Array(&CibouletteResourceSchema::String);
    let schema: CibouletteResourceSchema = CibouletteResourceSchema::Obj(
        vec![("hello".to_string(), &nested_schema)]
            .into_iter()
            .collect(),
    );
    let value = r#"
	{
		"hello": [
			1,
			2
		]
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .deserialize(&mut deserializer)
        .expect_err("the value type should produce an error");
}
