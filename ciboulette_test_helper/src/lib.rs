pub use ciboulette;
use ciboulette::*;
use messy_json::*;
use std::borrow::Cow;
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
//   - favorite_color
//   - articles
//   - comments

// comments:
// - attributes:
//   - body
// - relationships:
//   - author
//   - articles

// favorite_color:
// - attributes:
//   - color

fn gen_messy_json_schema_favorite_color<'a>() -> MessyJsonObject<'a> {
    MessyJsonObject::new(
        vec![(
            "color".to_string(),
            MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
        )]
        .into_iter()
        .collect(),
        false,
    )
}

fn gen_messy_json_schema_articles<'a>() -> MessyJsonObject<'a> {
    MessyJsonObject::new(
        vec![
            (
                "title".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
            ),
            (
                "body".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )
}

fn gen_messy_json_schema_comments<'a>() -> MessyJsonObject<'a> {
    MessyJsonObject::new(
        vec![(
            "body".to_string(),
            MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
        )]
        .into_iter()
        .collect(),
        false,
    )
}

fn gen_messy_json_schema_people_article<'a>() -> MessyJsonObject<'a> {
    MessyJsonObject::new(
        vec![
            (
                "people_id".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
            ),
            (
                "article_id".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )
}

fn gen_messy_json_schema_peoples<'a>() -> MessyJsonObject<'a> {
    MessyJsonObject::new(
        vec![
            (
                "first-name".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
            ),
            (
                "last-name".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
            ),
            (
                "age".to_string(),
                MessyJson::Number(Cow::Owned(MessyJsonNumeric::new(
                    MessyJsonNumberType::U64,
                    true,
                ))),
            ),
            (
                "gender".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
            (
                "twitter".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )
}

pub fn gen_bag<'a>() -> CibouletteStore<'a> {
    let mut res = CibouletteStore::new();

    res.add_type(
        "articles".to_string(),
        CibouletteIdType::Uuid,
        gen_messy_json_schema_articles(),
    )
    .unwrap();
    res.add_type(
        "comments".to_string(),
        CibouletteIdType::Uuid,
        gen_messy_json_schema_comments(),
    )
    .unwrap();
    res.add_type(
        "peoples".to_string(),
        CibouletteIdType::Uuid,
        gen_messy_json_schema_peoples(),
    )
    .unwrap();
    res.add_type(
        "favorite_color".to_string(),
        CibouletteIdType::Uuid,
        gen_messy_json_schema_favorite_color(),
    )
    .unwrap();

    res.add_type(
        "people-article".to_string(),
        CibouletteIdType::Uuid,
        gen_messy_json_schema_people_article(),
    )
    .unwrap();

    res.add_one_to_one_rel(
        ("peoples", None),
        ("favorite_color", None),
        CibouletteRelationshipOneToOneOption::new(
            "favorite_color".to_string(),
            CibouletteIdType::Uuid,
            true,
        ),
    )
    .unwrap(); // Articles -> Comments
    res.add_one_to_one_rel(
        ("comments", None),
        ("articles", None),
        CibouletteRelationshipOneToOneOption::new(
            "article_id".to_string(),
            CibouletteIdType::Uuid,
            false,
        ),
    )
    .unwrap(); // Articles -> Comments
    res.add_one_to_one_rel(
        ("comments", None),
        ("peoples", Some("author")),
        CibouletteRelationshipOneToOneOption::new(
            "author_id".to_string(),
            CibouletteIdType::Uuid,
            false,
        ),
    )
    .unwrap(); // Peoples -> Comments
    res.add_many_to_many_rel(
        ("articles", None),
        ("peoples", Some("author")),
        CibouletteRelationshipManyToManyOption::new(
            res.get_type("people-article").unwrap().clone(),
            [
                (res.get_type("articles").unwrap().clone(), "article_id".to_string()),
                (res.get_type("peoples").unwrap().clone(), "people_id".to_string()),
            ],
        ),
    )
    .unwrap(); // Peoples -> Articles
    res
}

pub fn check_ident<'a>(ident: &CibouletteResourceIdentifier<'a>, type_: &str, id: &CibouletteId) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_ident_permissive<'a>(
    ident: &CibouletteResourceIdentifierPermissive<'a>,
    type_: &str,
    id: &Option<CibouletteId>,
) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_single<'a, T>(
    selector: &'a CibouletteResourceSelector<'a, T>,
) -> &'a CibouletteResource<'a, T> {
    match selector {
        CibouletteResourceSelector::One(x) => x,
        _ => panic!("Expected a single resource"),
    }
}

pub fn check_multi<'a, T>(
    selector: &'a CibouletteResourceSelector<'a, T>,
) -> &'a Vec<CibouletteResource<'a, T>> {
    match selector {
        CibouletteResourceSelector::Many(x) => x,
        _ => panic!("Expected a multiple resources"),
    }
}
