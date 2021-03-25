use super::*;

#[test]
fn add_ok() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
}

#[test]
fn add_duplicate() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let err = store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UniqType(x) if x == "toto"),
        true
    );
}

#[test]
fn get_ok() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let type_ = store.get_type("toto");
    assert_eq!(matches!(type_, Ok(x) if x.name() == "toto"), true);
}

#[test]
fn get_not_found() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let err = store.get_type("aaaaaaaaaaaaaaaaaaa").unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(x) if x == "aaaaaaaaaaaaaaaaaaa"),
        true
    );
}

#[test]
fn get_optional_found() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let val = store.get_type_if_exists("toto");
    assert_eq!(matches!(val, Some(x) if x.name() == "toto"), true);
}

#[test]
fn get_optional_not_found() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let val = store.get_type_if_exists("aaaaaaaaaaa");
    assert_eq!(matches!(val, None), true);
}

#[test]
fn get_index() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let val = store.get_type_index("toto");
    let type_ = store.graph().node_weight(*val.unwrap()).unwrap();
    assert_eq!(type_.name(), "toto");
}

#[test]
fn get_index_not_found() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let val = store.get_type_index("aaaa");
    assert_eq!(matches!(val, None), true);
}

#[test]
fn get_type_with_index() {
    let mut store = CibouletteStore::default();

    store
        .add_type("toto", CibouletteIdType::Uuid, MessyJsonObject::default())
        .unwrap();
    let (index, type_) = store.get_type_with_index("toto").unwrap();
    let type_2 = store.graph().node_weight(index).unwrap();

    assert_eq!(type_.name(), "toto");
    assert_eq!(type_, type_2);
}
