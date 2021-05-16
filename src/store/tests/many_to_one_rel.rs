use super::*;

#[test]
fn ok() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "favorite_color",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        arcstr::literal!("id"),
        store.get_type("peoples").unwrap().clone(),
        arcstr::literal!("favorite_color"),
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
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "favorite_color",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        arcstr::literal!("id"),
        store.get_type("peoples").unwrap().clone(),
        arcstr::literal!("favorite_color"),
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
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "favorite_color",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        arcstr::literal!("id"),
        store.get_type("peoples").unwrap().clone(),
        arcstr::literal!("favorite_color"),
        true,
    );
    store.add_many_to_one_rel_no_reverse(opt, None).unwrap();
    let err = store.get_rel("favorite_color", "peoples").unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(x, y) if x == "favorite_color" && y == "peoples"),
        true
    );
    store.get_rel("peoples", "favorite_color").unwrap();
}

#[test]
fn duplicate() {
    let mut store = CibouletteStoreBuilder::default();

    store
        .add_type(
            "peoples",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_peoples(),
        )
        .unwrap();
    store
        .add_type(
            "favorite_color",
            CibouletteIdTypeSelector::Single(CibouletteIdType::Uuid(arcstr::literal!("id"))),
            crate::test_helper::gen_messy_json_schema_favorite_color(),
        )
        .unwrap();
    let opt = CibouletteRelationshipOneToManyOptionBuilder::new(
        store.get_type("favorite_color").unwrap().clone(),
        arcstr::literal!("id"),
        store.get_type("peoples").unwrap().clone(),
        arcstr::literal!("favorite_color"),
        true,
    );
    store.add_one_to_many_rel(opt.clone(), None, None).unwrap();
    let err = store.add_one_to_many_rel(opt, None, None).unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqRelationship(x, y) if x == "favorite_color" && y == "peoples"),
        true
    );
}
