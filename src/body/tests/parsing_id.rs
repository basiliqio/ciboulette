use super::*;
use std::borrow::Cow;
use std::str::FromStr;
use uuid::Uuid;

#[test]
fn text_to_uuid() {
    let builder = Cow::Borrowed("8278146c-b037-4364-8326-55db392e13a2");

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
            arcstr::literal!("id"),
        ))),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        matches!(res.get(0).unwrap().clone(), CibouletteId::Uuid(x) if x == Uuid::parse_str("8278146c-b037-4364-8326-55db392e13a2").unwrap()),
        true
    );
}

#[test]
fn text_to_number() {
    let builder = Cow::Borrowed("42");

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Number(
            arcstr::literal!("id"),
        ))),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        matches!(res.get(0).unwrap().clone(), CibouletteId::Number(x) if x == 42),
        true
    );
}

#[test]
fn text_to_text() {
    let builder = Cow::Owned(base64::encode("hello_world"));

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Text(
            arcstr::literal!("id"),
        ))),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        matches!(res.get(0).unwrap().clone(), CibouletteId::Text(x) if x == Cow::Borrowed("hello_world")),
        true
    );
}

#[test]
fn number_to_text() {
    let builder = Cow::Owned(base64::encode("42"));

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Text(
            arcstr::literal!("id"),
        ))),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        matches!(res.get(0).unwrap().clone(), CibouletteId::Text(x) if x.as_ref() == "42"),
        true
    );
}

#[test]
fn number_to_uuid() {
    let builder = Cow::Borrowed("42");

    let err = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
            arcstr::literal!("id"),
        ))),
        builder,
    )
    .unwrap_err();
    assert_eq!(matches!(err, CibouletteError::UuidError(_)), true);
}

#[test]
fn multi_text() {
    let builder = Cow::Owned(format!(
        "{},{},{}",
        base64::encode("hello_world"),
        base64::encode("toto"),
        base64::encode("tutu")
    ));

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Multi(vec![
            CibouletteIdType::Text(arcstr::literal!("aa")),
            CibouletteIdType::Text(arcstr::literal!("bb")),
            CibouletteIdType::Text(arcstr::literal!("cc")),
        ])),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 3);
    assert_eq!(
        matches!(res.get(0).unwrap(), CibouletteId::Text(x) if x.as_ref() == "hello_world"),
        true
    );

    assert_eq!(
        matches!(res.get(1).unwrap(), CibouletteId::Text(x) if x.as_ref() == "toto"),
        true
    );

    assert_eq!(
        matches!(res.get(2).unwrap(), CibouletteId::Text(x) if x.as_ref() == "tutu"),
        true
    );
}

#[test]
fn multi_mixed() {
    let builder = Cow::Owned(format!(
        "{},{},{}",
        42,
        "2e99cd9a-b93e-48df-b183-d219f390b8fd",
        base64::encode("tutu")
    ));

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Multi(vec![
            CibouletteIdType::Number(arcstr::literal!("aa")),
            CibouletteIdType::Uuid(arcstr::literal!("bb")),
            CibouletteIdType::Text(arcstr::literal!("cc")),
        ])),
        builder,
    )
    .unwrap();
    assert_eq!(res.len(), 3);
    assert_eq!(
        matches!(res.get(0).unwrap(), CibouletteId::Number(x) if *x == 42),
        true
    );

    assert_eq!(
        matches!(res.get(1).unwrap(), CibouletteId::Uuid(x) if x == &Uuid::from_str("2e99cd9a-b93e-48df-b183-d219f390b8fd").unwrap()),
        true
    );

    assert_eq!(
        matches!(res.get(2).unwrap(), CibouletteId::Text(x) if x.as_ref() == "tutu"),
        true
    );
}

#[test]
fn bad_utf8() {
    let builder = Cow::Borrowed("tutu");

    let res = CibouletteIdSelector::build_id(
        &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Text(
            arcstr::literal!("cc"),
        ))),
        builder,
    )
    .unwrap_err();

    assert_eq!(matches!(res, CibouletteError::FromUtf8(_)), true);
}
