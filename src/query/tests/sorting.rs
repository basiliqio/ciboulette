use super::*;

#[test]
fn single_asc() {
    let (bag, builder) = setup(r#"sort=first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
}

#[test]
fn single_asc_with_positive() {
    let (bag, builder) = setup(r#"sort=%2Bfirst-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
}

#[test]
fn single_desc() {
    let (bag, builder) = setup(r#"sort=-first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Desc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
}

#[test]
fn multiple_mixed() {
    let (bag, builder) = setup(r#"sort=last-name,-first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 2);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "last-name");
    assert_eq!(
        matches!(sorting[1].direction(), &CibouletteSortingDirection::Desc),
        true
    );
    assert_eq!(sorting[1].field(), "first-name");
}

#[test]
fn unknown_field() {
    let (bag, builder) = setup(r#"sort=unknown_field"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownField(type_, field) if type_.as_str() == "peoples" && field.as_str() == "unknown_field"),
        true
    );
}

#[test]
fn empty() {
    let (bag, builder) = setup(r#"sort="#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownField(type_, field) if type_.as_str() == "peoples" && field.as_str() == "<empty>"),
        true
    );
}
