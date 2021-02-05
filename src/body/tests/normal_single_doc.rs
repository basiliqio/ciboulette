use super::*;

#[test]
fn ok() {
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
			}
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc = CibouletteTopLevel::deserialize(&mut deserializer, &bag)
        .expect("to parse the json:api document");
    let data = check_single(&doc.data().as_ref().expect("data to be defined"));
    check_ident(
        data.identifier(),
        "english",
        "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
    );
    assert_eq!(
        data.attributes()
            .clone()
            .unwrap()
            .inner()
            .eq(&parse_attribute_english()),
        true,
        "attributes object mismatch"
    );
    println!("{:#?}", doc);
}
