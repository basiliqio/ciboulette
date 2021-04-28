pub use ciboulette;
use ciboulette::*;
use messy_json::*;
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

pub fn gen_messy_json_schema_favorite_color() -> MessyJsonObject {
    MessyJsonObject::from(MessyJsonObjectInner::new(
        vec![(
            arcstr::literal!("color"),
            MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
        )]
        .into_iter()
        .collect(),
        false,
    ))
}

pub fn gen_messy_json_schema_articles() -> MessyJsonObject {
    MessyJsonObject::from(MessyJsonObjectInner::new(
        vec![
            (
                arcstr::literal!("title"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
            ),
            (
                arcstr::literal!("body"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(true))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    ))
}

pub fn gen_messy_json_schema_comments() -> MessyJsonObject {
    MessyJsonObject::from(MessyJsonObjectInner::new(
        vec![(
            arcstr::literal!("body"),
            MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
        )]
        .into_iter()
        .collect(),
        false,
    ))
}

pub fn gen_messy_json_schema_people_article() -> MessyJsonObject {
    MessyJsonObject::from(MessyJsonObjectInner::new(
        vec![
            (
                arcstr::literal!("people_id"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
            ),
            (
                arcstr::literal!("article_id"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    ))
}

pub fn gen_messy_json_schema_peoples() -> MessyJsonObject {
    MessyJsonObject::from(MessyJsonObjectInner::new(
        vec![
            (
                arcstr::literal!("first-name"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
            ),
            (
                arcstr::literal!("last-name"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
            ),
            (
                arcstr::literal!("age"),
                MessyJson::from(MessyJsonInner::Number(MessyJsonNumeric::new(
                    MessyJsonNumberType::U64,
                    true,
                ))),
            ),
            (
                arcstr::literal!("gender"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(true))),
            ),
            (
                arcstr::literal!("twitter"),
                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(true))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    ))
}

pub fn gen_bag() -> CibouletteStore {
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
            arcstr::literal!("favorite_color"),
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
            arcstr::literal!("article"),
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
            arcstr::literal!("author"),
			false
        ),
        Some(arcstr::ArcStr::from("author")),
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
                    arcstr::literal!("article_id"),
                ),
                (
                    res.get_type("peoples").unwrap().clone(),
                    arcstr::literal!("people_id"),
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

pub fn check_single<'request,MessyJsonObjectValue, T>(
    selector: &'request CibouletteResourceSelector<'request,MessyJsonObjectValue, T>,
) -> &'request CibouletteResource<'request,MessyJsonObjectValue, T> {
    match selector {
        CibouletteResourceSelector::One(x) => x,
        _ => panic!("Expected a single resource"),
    }
}

pub fn check_multi<'request, T>(
    selector: &'request CibouletteResourceSelector<'request, MessyJsonObjectValue<'request>, T>,
) -> &'request Vec<CibouletteResource<'request, MessyJsonObjectValue<'request>, T>> {
    match selector {
        CibouletteResourceSelector::Many(x) => x,
        _ => panic!("Expected a multiple resources"),
    }
}
