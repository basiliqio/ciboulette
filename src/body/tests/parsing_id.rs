use super::*;
use std::borrow::Cow;
use uuid::Uuid;

#[test]
fn text_to_uuid() {
    let builder = CibouletteIdBuilder::Text(Cow::Borrowed("8278146c-b037-4364-8326-55db392e13a2"));

    let res = builder.build(&CibouletteIdType::Uuid).unwrap();
    assert_eq!(
        matches!(res, CibouletteId::Uuid(x) if x == Uuid::parse_str("8278146c-b037-4364-8326-55db392e13a2").unwrap()),
        true
    );
}

#[test]
fn text_to_number() {
    let builder = CibouletteIdBuilder::Text(Cow::Borrowed("42"));

    let res = builder.build(&CibouletteIdType::Number).unwrap();
    assert_eq!(matches!(res, CibouletteId::Number(x) if x == 42), true);
}

#[test]
fn text_to_text() {
    let builder = CibouletteIdBuilder::Text(Cow::Borrowed("hello_world"));

    let res = builder.build(&CibouletteIdType::Text).unwrap();
    assert_eq!(
        matches!(res, CibouletteId::Text(x) if x == Cow::Borrowed("hello_world")),
        true
    );
}

#[test]
fn number_to_text() {
    let builder = CibouletteIdBuilder::Number(42);

    let res = builder.build(&CibouletteIdType::Text).unwrap();
    assert_eq!(
        matches!(res, CibouletteId::Text(x) if x.as_ref() == "42"),
        true
    );
}

#[test]
fn number_to_number() {
    let builder = CibouletteIdBuilder::Number(42);

    let res = builder.build(&CibouletteIdType::Number).unwrap();
    assert_eq!(matches!(res, CibouletteId::Number(x) if x == 42), true);
}

#[test]
fn number_to_uuid() {
    let builder = CibouletteIdBuilder::Number(42);

    let err = builder.build(&CibouletteIdType::Uuid).unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::BadIdType(x, y) if x == CibouletteIdType::Number && y == CibouletteIdType::Uuid),
        true
    );
}
