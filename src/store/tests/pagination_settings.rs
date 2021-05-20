use super::*;

#[test]
fn mismatching_discriminant() {
    let builder = CibouletteResourceTypePaginationConfigurationBuilder {
        sort_by: vec!["title".to_string(), "id".to_string()],
        discriminants: vec!["author".to_string()],
        encoding: CiboulettePaginationEncoding::Base64,
    };

    let res = builder
        .build(
            "toto",
            &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
                arcstr::literal!("id"),
            ))),
            &crate::test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap_err();
    assert_eq!(
        matches!(res, CibouletteError::UnknownPaginationField(_, x) if x == "author"),
        true
    );
}

#[test]
fn unknown_sorting_field() {
    let builder = CibouletteResourceTypePaginationConfigurationBuilder {
        sort_by: vec!["title".to_string(), "aaaa".to_string()],
        discriminants: vec!["author".to_string()],
        encoding: CiboulettePaginationEncoding::Base64,
    };

    let res = builder
        .build(
            "toto",
            &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
                arcstr::literal!("id"),
            ))),
            &crate::test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap_err();
    assert_eq!(
        matches!(res, CibouletteError::UnknownField(_, x) if x == "aaaa"),
        true
    );
}

#[test]
fn unknown_discriminant_field() {
    let builder = CibouletteResourceTypePaginationConfigurationBuilder {
        sort_by: vec!["title".to_string(), "id".to_string()],
        discriminants: vec!["aaaa".to_string()],
        encoding: CiboulettePaginationEncoding::Base64,
    };

    let res = builder
        .build(
            "toto",
            &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
                arcstr::literal!("id"),
            ))),
            &crate::test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap_err();
    assert_eq!(
        matches!(res, CibouletteError::UnknownPaginationField(_, x) if x == "aaaa"),
        true
    );
}

#[test]
fn multiple_same_sort_fields() {
    let builder = CibouletteResourceTypePaginationConfigurationBuilder {
        sort_by: vec![
            "title".to_string(),
            "title".to_string(),
            "title".to_string(),
            "id".to_string(),
        ],
        discriminants: vec!["id".to_string()],
        encoding: CiboulettePaginationEncoding::Base64,
    };

    builder
        .build(
            "toto",
            &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
                arcstr::literal!("id"),
            ))),
            &crate::test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
}

#[test]
fn multiple_same_discriminant_fields() {
    let builder = CibouletteResourceTypePaginationConfigurationBuilder {
        sort_by: vec!["title".to_string(), "id".to_string()],
        discriminants: vec!["id".to_string(), "id".to_string(), "id".to_string()],
        encoding: CiboulettePaginationEncoding::Base64,
    };

    builder
        .build(
            "toto",
            &CibouletteIdTypeSelector::new(CibouletteSelector::Single(CibouletteIdType::Uuid(
                arcstr::literal!("id"),
            ))),
            &crate::test_helper::gen_messy_json_schema_articles(),
        )
        .unwrap();
}
