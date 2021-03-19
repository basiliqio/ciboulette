use super::*;

#[test]
fn normal() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		},
		"links":
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
    assert_eq!(doc.is_compound(), false);
    assert_eq!(doc.has_all_ids(), true);
    assert_eq!(doc.has_data(), true);
}

#[test]
fn no_data() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"errors":
		{
			"id": "toto",
			"status": 400
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect("to build the document");
    assert_eq!(doc.is_compound(), false);
    assert_eq!(doc.has_all_ids(), true);
    assert_eq!(doc.has_data(), false);
}

#[test]
fn has_all_ids() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		{
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		},
		"links":
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
    assert_eq!(doc.is_compound(), false);
    assert_eq!(doc.has_all_ids(), false);
    assert_eq!(doc.has_data(), true);
}

#[test]
fn compound_simple() {
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
			},
			{
				"id": "44814756-fdff-4d8f-a7d3-6fb11184af81",
				"type": "comments",
				"attributes":
				{
					"body": "world2"
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
    assert_eq!(doc.is_compound(), true);
    assert_eq!(doc.has_all_ids(), true);
    assert_eq!(doc.has_data(), true);
}

#[test]
fn compound_missing_ids() {
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
			},
			{
				"type": "comments",
				"attributes":
				{
					"body": "world2"
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
    assert_eq!(doc.is_compound(), true);
    assert_eq!(doc.has_all_ids(), false);
    assert_eq!(doc.has_data(), true);
}

#[test]
fn compound_empty() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data":
		[
		]
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let doc_builder = CibouletteBodyBuilder::deserialize(&mut deserializer)
        .expect("to parse the json:api document");
    let doc = doc_builder
        .build(&bag, &CibouletteIntention::Read)
        .expect("to build the document");
    assert_eq!(doc.is_compound(), true);
    assert_eq!(doc.has_all_ids(), true);
    assert_eq!(doc.has_data(), true);
}
