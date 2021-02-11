use super::*;

// Articles:
// - attributes:
//   - title
//   - body?
// - relationships:
//   - author -> people
//   - comments

// People:
// - attributes:
//   - first-name
//   - last-name
//   - age?
//   - gender?
//   - twitter?
// - relationships:
//   - articles
//   - comments

// comments:
// - attributes:
//   - body
// - relationships:
//   - author
//   - articles

fn gen_messy_json_schema_articles() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![
            (
                "title".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "body".to_string(),
                MessyJson::String(MessyJsonScalar::new(true)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )))
}

fn gen_messy_json_schema_comments() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![(
            "body".to_string(),
            MessyJson::String(MessyJsonScalar::new(false)),
        )]
        .into_iter()
        .collect(),
        false,
    )))
}

fn gen_messy_json_schema_peoples() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![
            (
                "first-name".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "last-name".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "age".to_string(),
                MessyJson::String(MessyJsonScalar::new(true)),
            ),
            (
                "gender".to_string(),
                MessyJson::String(MessyJsonScalar::new(true)),
            ),
            (
                "twitter".to_string(),
                MessyJson::String(MessyJsonScalar::new(true)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )))
}

// fn gen_messy_json_schema2() -> MessyJson {
//     MessyJson::Obj(Box::new(MessyJsonObject::new(
//         vec![
//             (
//                 "bonjour".to_string(),
//                 MessyJson::String(MessyJsonScalar::new(false)),
//             ),
//             (
//                 "monde".to_string(),
//                 MessyJson::String(MessyJsonScalar::new(false)),
//             ),
//         ]
//         .into_iter()
//         .collect(),
//         false,
//     )))
// }

// fn gen_messy_json_schema3() -> MessyJson {
//     MessyJson::Obj(Box::new(MessyJsonObject::new(
//         vec![(
//             "p".to_string(),
//             MessyJson::String(MessyJsonScalar::new(false)),
//         )]
//         .into_iter()
//         .collect(),
//         false,
//     )))
// }

pub fn gen_bag() -> CibouletteBag {
    let type1 = CibouletteResourceType::new(
        "articles".to_string(),
        gen_messy_json_schema_articles(),
        vec![
            ("author".to_string(), "peoples".to_string()),
            ("comments".to_string(), "comments".to_string()),
        ],
    );
    let type2 = CibouletteResourceType::new(
        "comments".to_string(),
        gen_messy_json_schema_comments(),
        vec![
            ("author".to_string(), "peoples".to_string()),
            ("articles".to_string(), "articles".to_string()),
        ],
    );
    let type3 = CibouletteResourceType::new(
        "peoples".to_string(),
        gen_messy_json_schema_peoples(),
        vec![
            ("comments".to_string(), "comments".to_string()),
            ("articles".to_string(), "articles".to_string()),
        ],
    );
    CibouletteBag::new(
        vec![
            ("articles".to_string(), type1),
            ("comments".to_string(), type2),
            ("peoples".to_string(), type3),
        ]
        .into_iter()
        .collect(),
    )
}

pub fn check_ident(ident: &CibouletteResourceIdentifier, type_: &str, id: &str) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_single<'a>(
    selector: &'a CibouletteResourceSelector<'a>,
) -> &'a CibouletteResource<'a> {
    match selector {
        CibouletteResourceSelector::One(x) => x,
        _ => panic!("Expected a single resource"),
    }
}

pub fn check_multi<'a>(
    selector: &'a CibouletteResourceSelector<'a>,
) -> &'a Vec<CibouletteResource<'a>> {
    match selector {
        CibouletteResourceSelector::Many(x) => x,
        _ => panic!("Expected a multiple resources"),
    }
}
