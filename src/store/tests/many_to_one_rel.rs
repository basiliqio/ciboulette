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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        store.get_type("peoples").unwrap().clone(),
        "favorite_color".to_string(),
        true,
    );
    store.add_one_to_many_rel(opt.clone(), None, None).unwrap();
    let rel = store.get_rel("peoples", "favorite_color").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::ManyToOne(x) if x == &opt),
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        store.get_type("peoples").unwrap().clone(),
        "favorite_color".to_string(),
        true,
    );
    store.add_one_to_many_rel(opt.clone(), None, None).unwrap();
    let rel = store.get_rel("favorite_color", "peoples").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOptionBuilder::OneToMany(x) if x == &opt),
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        store.get_type("peoples").unwrap().clone(),
        "favorite_color".to_string(),
        true,
    );
    store
        .add_one_to_many_rel_no_reverse(opt.clone(), None)
        .unwrap();
    let err = store.get_rel("peoples", "favorite_color").unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(x, y) if x == "peoples" && y == "favorite_color"),
        true
    );
    store.get_rel("favorite_color", "peoples").unwrap();
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        store.get_type("peoples").unwrap().clone(),
        "favorite_color".to_string(),
        true,
    );
    store.add_one_to_many_rel(opt.clone(), None, None).unwrap();
    let err = store
        .add_one_to_many_rel(opt.clone(), None, None)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "favorite_color" && y == "peoples"),
        true
    );
}
