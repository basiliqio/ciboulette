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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToOneOption::new("color", CibouletteIdType::Uuid, false);
    store
        .add_one_to_one_rel(("peoples", None), ("favorite_color", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("peoples", "favorite_color").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOption::OneToOne(x) if x == &opt),
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToOneOption::new("color", CibouletteIdType::Uuid, false);
    store
        .add_one_to_one_rel(("peoples", None), ("favorite_color", None), opt.clone())
        .unwrap();
    let rel = store.get_rel("favorite_color", "peoples").unwrap();
    assert_eq!(
        matches!(rel.1, CibouletteRelationshipOption::OneToOne(x) if x == &opt),
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToOneOption::new("color", CibouletteIdType::Uuid, false);
    store
        .add_one_to_one_rel(("peoples", None), ("favorite_color", None), opt.clone())
        .unwrap();
    let err = store
        .add_one_to_one_rel(("peoples", None), ("favorite_color", None), opt)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "peoples" && y == "favorite_color"),
        true
    );
}

#[test]
fn unknown_type_1() {
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToOneOption::new("color", CibouletteIdType::Uuid, false);
    let err = store
        .add_one_to_one_rel(("aaa", None), ("favorite_color", None), opt)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(x) if x == "aaa"),
        true
    );
}

#[test]
fn unknown_type_2() {
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
            "favorite_color",
            CibouletteIdType::Uuid,
            ciboulette_test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToOneOption::new("color", CibouletteIdType::Uuid, false);
    let err = store
        .add_one_to_one_rel(("peoples", None), ("aaa", None), opt)
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(x) if x == "aaa"),
        true
    );
}
