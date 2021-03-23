use super::*;

#[test]
fn ok() {
    let mut store = CibouletteStore::new();

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
    let opt = CibouletteRelationshipManyToManyOption::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                "people_id".to_string(),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                "article_id".to_string(),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("peoples", "articles").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOption::ManyToMany(x) if x == &opt),
        true
    );
}

#[test]
fn ok_reverse() {
    let mut store = CibouletteStore::new();

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
    let opt = CibouletteRelationshipManyToManyOption::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                "people_id".to_string(),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                "article_id".to_string(),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("articles", "author").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOption::ManyToMany(x) if x == &opt),
        true
    );
}

#[test]
fn duplicate() {
    let mut store = CibouletteStore::new();

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
    let opt = CibouletteRelationshipManyToManyOption::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                "people_id".to_string(),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                "article_id".to_string(),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap();
    let err = store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "peoples" && y == "articles"),
        true
    );
}

#[test]
fn alias() {
    let mut store = CibouletteStore::new();

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
    let opt = CibouletteRelationshipManyToManyOption::new(
        store.get_type("people-article").unwrap().clone(),
        [
            (
                store.get_type("peoples").unwrap().clone(),
                "people_id".to_string(),
            ),
            (
                store.get_type("articles").unwrap().clone(),
                "article_id".to_string(),
            ),
        ],
    );
    store
        .add_many_to_many_rel(("peoples", Some("author")), ("articles", None), opt.clone())
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
