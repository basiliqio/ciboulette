use super::*;

#[test]
fn single_ok() {
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
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect("to build the document");
    let data = match doc.data() {
        CibouletteBodyData::Object(x) => x,
        _ => panic!("data should've been present"),
    };
    let data = check_multi(data);
    check_ident_permissive(
        data.get(0).unwrap().identifier(),
        "comments",
        Some(CibouletteIdSelector::new(CibouletteSelector::Single(
            CibouletteId::Uuid(
                uuid::Uuid::parse_str("6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap(),
            ),
        ))),
    );
    assert_eq!(
        data.get(0)
            .unwrap()
            .attributes()
            .clone()
            .unwrap()
            .eq(&parse_attribute_comments()),
        true,
        "attributes object mismatch"
    );
    println!("{:#?}", doc);
}

#[test]
fn single_unknown_type() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "commentse",
				"attributes":
				{
					"body": "world"
				}
			}
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc: CibouletteError = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect_err("the type should be unknown");
    if let CibouletteError::UnknownType(_) = doc {
    } else {
        panic!("Wrong error");
    }
    println!("{:#?}", doc);
}
