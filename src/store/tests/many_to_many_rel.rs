use super::*;

#[test]
fn ok() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "articles",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
    store
        .add_type(
            "people-article",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_people_article(),
        )
        .unwrap();
    let opt = CibouletteRelationshipManyToManyOptionBuilder::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                arcstr::literal!("people_id"),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                arcstr::literal!("article_id"),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("peoples", "articles").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::ManyToMany(x) if x == &opt),
        true
    );
}

#[test]
fn ok_reverse() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "articles",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
    store
        .add_type(
            "people-article",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_people_article(),
        )
        .unwrap();
    let opt = CibouletteRelationshipManyToManyOptionBuilder::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                arcstr::literal!("people_id"),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                arcstr::literal!("article_id"),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("articles", "author").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::ManyToMany(x) if x == &opt),
        true
    );
}

#[test]
fn no_reverse() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "articles",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
    store
        .add_type(
            "people-article",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_people_article(),
        )
        .unwrap();
    let opt = CibouletteRelationshipManyToManyOptionBuilder::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                arcstr::literal!("people_id"),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                arcstr::literal!("article_id"),
            ),
        ],
    );
    store
        .add_many_to_many_rel_no_reverse("peoples", ("articles", None), opt)
        .unwrap();
    let err = store.get_rel("articles", "author").unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(x, y) if x == "articles" && y == "author"),
        true
    );
    store.get_rel("peoples", "articles").unwrap();
}

#[test]
fn duplicate() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "articles",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
    store
        .add_type(
            "people-article",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_people_article(),
        )
        .unwrap();
    let opt = CibouletteRelationshipManyToManyOptionBuilder::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                arcstr::literal!("people_id"),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                arcstr::literal!("article_id"),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let err = store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "peoples" && y == "articles"),
        true
    );
}

#[test]
fn alias() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "articles",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
    store
        .add_type(
            "people-article",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_people_article(),
        )
        .unwrap();
    let opt = CibouletteRelationshipManyToManyOptionBuilder::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                arcstr::literal!("people_id"),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                arcstr::literal!("article_id"),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt)
        .unwrap();
    assert_eq!(
        store
            .get_type("peoples")
            .unwrap()
            .get_alias("people-article")
            .unwrap(),
        "people-article"
    );
    assert_eq!(
        store
            .get_type("peoples")
            .unwrap()
            .get_alias("articles")
            .unwrap(),
        "articles"
    );
    assert_eq!(
        store
            .get_type("articles")
            .unwrap()
            .get_alias("peoples")
            .unwrap(),
        "author"
    );
}
