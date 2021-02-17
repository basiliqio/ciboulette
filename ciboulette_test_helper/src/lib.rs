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

fn gen_messy_json_schema_article_comments() -> MessyJson {
	MessyJson::Obj(Box::new(MessyJsonObject::new(
		vec![(
			"article_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		),
		(
			"comment_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		)
		]
		.into_iter()
		.collect(),
		false,
	)))
}

fn gen_messy_json_schema_people_article() -> MessyJson {
	MessyJson::Obj(Box::new(MessyJsonObject::new(
		vec![(
			"people_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		),
		(
			"article_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		)
		]
		.into_iter()
		.collect(),
		false,
	)))
}

fn gen_messy_json_schema_people_comments() -> MessyJson {
	MessyJson::Obj(Box::new(MessyJsonObject::new(
		vec![(
			"people_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		),
		(
			"comment_id".to_string(),
			MessyJson::String(MessyJsonScalar::new(false)),
		)
		]
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

pub fn gen_bag() -> CibouletteStore {
	let mut res = CibouletteStore::new();

	res.add_type("articles".to_string(), gen_messy_json_schema_articles())
		.unwrap();
	res.add_type("comments".to_string(), gen_messy_json_schema_comments())
		.unwrap();
	res.add_type("peoples".to_string(), gen_messy_json_schema_peoples())
		.unwrap();
	res.add_type(
		"article-comment".to_string(),
		gen_messy_json_schema_article_comments(),
	)
	.unwrap();
	res.add_type(
		"people-article".to_string(),
		gen_messy_json_schema_people_article(),
	)
	.unwrap();

	res.add_type(
		"people-comment".to_string(),
		gen_messy_json_schema_people_comments(),
	)
	.unwrap();

	res.add_rel(
		("articles", None),
		("comments", None),
		CibouletteRelationshipOption::Many(CibouletteRelationshipBucket::new(
			res.get_type("article-comment").unwrap().clone(),
			"article_id".to_string(),
			"comment_id".to_string(),
		)),
	)
	.unwrap(); // Articles -> Comments
	res.add_rel(
		("peoples", Some("author")),
		("comments", None),
		CibouletteRelationshipOption::Many(CibouletteRelationshipBucket::new(
			res.get_type("people-comment").unwrap().clone(),
			"people_id".to_string(),
			"comment_id".to_string(),
		))	)
	.unwrap(); // Peoples -> Comments
	res.add_rel(
		("peoples", Some("author")),
		("articles", None),
		CibouletteRelationshipOption::Many(CibouletteRelationshipBucket::new(
			res.get_type("people-article").unwrap().clone(),
			"people_id".to_string(),
			"article_id".to_string(),
		)),
	)
	.unwrap(); // Peoples -> Articles
	res
}

pub fn check_ident(ident: &CibouletteResourceIdentifier, type_: &str, id: &str) {
	assert_eq!(ident.id(), id, "`id`s mismatch");
	assert_eq!(ident.type_(), type_, "`type`s mismatch");
}

pub fn check_ident_creator(
	ident: &CibouletteResourceIdentifierCreator,
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
