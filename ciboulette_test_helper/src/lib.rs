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

pub fn gen_messy_json_schema_favorite_color<'request>() -> MessyJsonObject<'request> {
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

pub fn gen_messy_json_schema_articles<'request>() -> MessyJsonObject<'request> {
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

pub fn gen_messy_json_schema_comments<'request>() -> MessyJsonObject<'request> {
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

pub fn gen_messy_json_schema_people_article<'request>() -> MessyJsonObject<'request> {
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

pub fn gen_messy_json_schema_peoples<'request>() -> MessyJsonObject<'request> {
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

pub fn gen_bag<'request>() -> CibouletteStore<'request> {
    let mut res = CibouletteStoreBuilder::default();

    res.add_type(
        "articles",
        CibouletteIdType::Uuid,
        gen_messy_json_schema_articles(),
    )
    .unwrap();
    res.add_type(
        "comments",
        CibouletteIdType::Uuid,
        gen_messy_json_schema_comments(),
    )
    .unwrap();
    res.add_type(
        "peoples",
        CibouletteIdType::Uuid,
        gen_messy_json_schema_peoples(),
    )
    .unwrap();
    res.add_type(
        "favorite_color",
        CibouletteIdType::Uuid,
        gen_messy_json_schema_favorite_color(),
    )
    .unwrap();

    res.add_type(
        "people-article",
        CibouletteIdType::Uuid,
        gen_messy_json_schema_people_article(),
    )
    .unwrap();

    res.add_one_to_many_rel(
        CibouletteRelationshipOneToManyOptionBuilder::new(
            res.get_type("favorite_color").unwrap().clone(),
            res.get_type("peoples").unwrap().clone(),
            "favorite_color".to_string(),
			true,
        ),
        None,
		None
    )
    .unwrap(); // Articles -> Comments
    res.add_one_to_many_rel(
        CibouletteRelationshipOneToManyOptionBuilder::new(
            res.get_type("articles").unwrap().clone(),
            res.get_type("comments").unwrap().clone(),
            "article".to_string(),
			false,
        ),
        None,
        None,
    )
    .unwrap(); // Peoples -> Comments
    res.add_one_to_many_rel(
        CibouletteRelationshipOneToManyOptionBuilder::new(
            res.get_type("peoples").unwrap().clone(),
            res.get_type("comments").unwrap().clone(),
            "author".to_string(),
			false
        ),
        Some("author"),
        None,
    )
    .unwrap(); // Peoples -> Comments
    res.add_many_to_many_rel(
        ("articles", None),
        ("peoples", Some("author")),
        CibouletteRelationshipManyToManyOptionBuilder::new(
            res.get_type("people-article").unwrap().clone(),
            [
                (
                    res.get_type("articles").unwrap().clone(),
                    "article_id".to_string(),
                ),
                (
                    res.get_type("peoples").unwrap().clone(),
                    "people_id".to_string(),
                ),
            ],
        ),
    )
    .unwrap(); // Peoples -> Articles
    res.build().unwrap()
}

pub fn check_ident<'request>(ident: &CibouletteResourceIdentifier<'request>, type_: &str, id: &CibouletteId) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_ident_permissive<'request>(
    ident: &CibouletteResourceIdentifierPermissive<'request>,
    type_: &str,
    id: &Option<CibouletteId>,
) {
    assert_eq!(ident.id(), id, "`id`s mismatch");
    assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_single<'request, 'store, MessyJsonObjectValue, T>(
    selector: &'request CibouletteResourceSelector<'request, 'store, MessyJsonObjectValue, T>,
) -> &'request CibouletteResource<'request, 'store, MessyJsonObjectValue, T> {
    match selector {
        CibouletteResourceSelector::One(x) => x,
        _ => panic!("Expected a single resource"),
    }
}

pub fn check_multi<'request, 'store, T>(
    selector: &'request CibouletteResourceSelector<'request, 'store, MessyJsonObjectValue<'store>, T>,
) -> &'request Vec<CibouletteResource<'request, 'store, MessyJsonObjectValue<'store>, T>> {
    match selector {
        CibouletteResourceSelector::Many(x) => x,
        _ => panic!("Expected a multiple resources"),
    }
}
