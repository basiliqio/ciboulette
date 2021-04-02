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
            "comments",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_comments(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("peoples").unwrap().clone(),
        store.get_type("comments").unwrap().clone(),
        "author".to_string(),
        false,
    );
    store
        .add_one_to_many_rel(opt.clone(), Some("author"), None)
        .unwrap();
    let rel = store.get_rel("peoples", "comments").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::OneToMany(x) if x == &opt),
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
            "comments",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_comments(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("peoples").unwrap().clone(),
        store.get_type("comments").unwrap().clone(),
        "author".to_string(),
        false,
    );
    store
        .add_one_to_many_rel(opt.clone(), Some("author"), None)
        .unwrap();
    let rel = store.get_rel("comments", "author").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::ManyToOne(x) if x == &opt),
        true
    );
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
            "comments",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_comments(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("peoples").unwrap().clone(),
        store.get_type("comments").unwrap().clone(),
        "author".to_string(),
        false,
    );
    store
        .add_one_to_many_rel(opt.clone(), Some("author"), None)
        .unwrap();
    let err = store
        .add_one_to_many_rel(opt.clone(), Some("author"), None)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "peoples" && y == "comments"),
        true
    );
}
