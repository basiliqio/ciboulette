mod bogus_deserialize;
mod check_linkage;
mod check_uniqueness;
mod normal_multi_docs;
mod normal_single_doc;

use super::*;

fn gen_messy_json_schema1() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "world".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )))
}

fn gen_messy_json_schema2() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![
            (
                "bonjour".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "monde".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )))
}

fn gen_messy_json_schema3() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![(
            "p".to_string(),
            MessyJson::String(MessyJsonScalar::new(false)),
        )]
        .into_iter()
        .collect(),
        false,
    )))
}

fn gen_bag() -> CibouletteBag {
    let type1 = CibouletteResourceType::new(gen_messy_json_schema1(), vec!["planet".to_string()]);
    let type2 = CibouletteResourceType::new(gen_messy_json_schema2(), vec!["planet".to_string()]);
    let type3 = CibouletteResourceType::new(gen_messy_json_schema3(), vec![]);
    CibouletteBag::new(
        vec![
            ("english".to_string(), type1),
            ("french".to_string(), type2),
            ("planet".to_string(), type3),
        ]
        .into_iter()
        .collect(),
    )
}

fn check_ident(ident: &CibouletteResourceIdentifier, type_: &str, id: &str) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

fn check_single<'a>(selector: &'a CibouletteResourceSelector<'a>) -> &'a CibouletteResource<'a> {
    match selector {
        CibouletteResourceSelector::One(x) => x,
        _ => panic!("Expected a single resource"),
    }
}

fn check_multi<'a>(
    selector: &'a CibouletteResourceSelector<'a>,
) -> &'a Vec<CibouletteResource<'a>> {
    match selector {
        CibouletteResourceSelector::Many(x) => x,
        _ => panic!("Expected a multiple resources"),
    }
}

fn parse_attribute_english() -> serde_json::Value {
    let s = r#"{
		"hello": "world",
		"world": "the earth"
	}"#;
    serde_json::from_str(s).unwrap()
}
