use super::*;

#[test]
fn single_element() {
    const DATA: &str = r#"
	{
		"hello": "world"
	}
	"#;

    let parsed: DummyStruct = serde_json::from_str(DATA).unwrap();
    assert_eq!(
        matches!(parsed.hello, CibouletteSelector::Single(x) if x == "world"),
        true
    );
}

#[test]
fn multiple_elements() {
    const DATA: &str = r#"
	{
		"hello": ["world", "toto"]
	}
	"#;

    let parsed: DummyStruct = serde_json::from_str(DATA).unwrap();
    assert_eq!(
        matches!(parsed.hello, CibouletteSelector::Multi(x) if x.len() == 2 && x == vec!["world", "toto"]),
        true
    );
}

#[test]
fn multiple_empty_elements() {
    const DATA: &str = r#"
	{
		"hello": []
	}
	"#;

    let parsed: DummyStruct = serde_json::from_str(DATA).unwrap();
    assert_eq!(
        matches!(parsed.hello, CibouletteSelector::Multi(x) if x.is_empty()),
        true
    );
}
