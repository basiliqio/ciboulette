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
				MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
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

	res.add_type("articles".to_string(), gen_messy_json_schema_articles())
		.unwrap();
	res.add_type("comments".to_string(), gen_messy_json_schema_comments())
		.unwrap();
	res.add_type("peoples".to_string(), gen_messy_json_schema_peoples())
		.unwrap();
	res.add_type(
		"favorite_color".to_string(),
		gen_messy_json_schema_favorite_color(),
	)
	.unwrap();

	res.add_type(
		"people-article".to_string(),
		gen_messy_json_schema_people_article(),
	)
	.unwrap();

	res.add_rel_single(
		("peoples", None),
		("favorite_color", None),
		CibouletteRelationshipOneToOneOption::new("favorite_color".to_string(), true),
	)
	.unwrap(); // Articles -> Comments
	res.add_rel_single(
		("comments", None),
		("articles", None),
		CibouletteRelationshipOneToOneOption::new("article_id".to_string(), false),
	)
	.unwrap(); // Articles -> Comments
	res.add_rel_single(
		("comments", None),
		("peoples", Some("author")),
		CibouletteRelationshipOneToOneOption::new("author_id".to_string(), false),
	)
	.unwrap(); // Peoples -> Comments
	res.add_rel_multiple(
		("articles", None),
		("peoples", Some("author")),
		CibouletteRelationshipBucket::new(
			res.get_type("people-article").unwrap().clone(),
			"article_id".to_string(),
			"people_id".to_string(),
		),
	)
	.unwrap(); // Peoples -> Articles
	res
}

pub fn check_ident<'a>(ident: &CibouletteResourceIdentifier<'a>, type_: &str, id: &str) {
	assert_eq!(ident.id(), id, "`id`s mismatch");
	assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_ident_permissive<'a>(
	ident: &CibouletteResourceIdentifierPermissive<'a>,
	type_: &str,
	id: Option<&str>,
) {
	assert_eq!(
		ident.id().as_ref(),
		id.map(std::borrow::Cow::Borrowed).as_ref(),
		"`id`s mismatch"
	);
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
