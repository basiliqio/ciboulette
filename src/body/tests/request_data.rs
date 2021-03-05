use super::*;

#[test]
fn null() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data": null,
		"meta":
		{
			"self": "comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect("to build the document");
    match doc.data() {
        CibouletteBodyData::Null(x) => assert_eq!(x, &true),
        _ => panic!("data should've been present"),
    };
    println!("{:#?}", doc);
}

#[test]
fn absent() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"meta":
		{
			"self": "comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect("to build the document");
    match doc.data() {
        CibouletteBodyData::Null(x) => assert_eq!(x, &false),
        _ => panic!("data should've been present"),
    };
    println!("{:#?}", doc);
}
