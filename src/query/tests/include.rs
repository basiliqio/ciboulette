use super::*;

#[test]
fn single() {
    let (bag, builder) = setup(r#"include=comments"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(!res.include().is_empty(), true);
    let include = res.include();
    assert_eq!(include.len(), 1);
    assert_eq!(
        include
            .get(bag.get_type("comments").unwrap().as_ref())
            .is_some(),
        true
    );
}

#[test]
fn multiple() {
    let (bag, builder) = setup(r#"include=comments,articles"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(!res.include().is_empty(), true);
    let include = res.include();
    assert_eq!(include.len(), 2);
    assert_eq!(
        include
            .get(bag.get_type("comments").unwrap().as_ref())
            .is_some(),
        true
    );
    assert_eq!(
        include
            .get(bag.get_type("articles").unwrap().as_ref())
            .is_some(),
        true
    );
}

#[test]
fn single_with_nesting() {
    let (bag, builder) = setup(r#"include=peoples.comments"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(!res.include().is_empty(), true);
    let include = res.include();
    assert_eq!(include.len(), 1);
    assert_eq!(
        include
            .get(bag.get_type("comments").unwrap().as_ref())
            .is_some(),
        true
    );
}

#[test]
fn multiple_with_nesting() {
    let (bag, builder) = setup(r#"include=peoples.comments,peoples.articles"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(!res.include().is_empty(), true);
    let include = res.include();
    assert_eq!(include.len(), 2);
    assert_eq!(
        include
            .get(bag.get_type("comments").unwrap().as_ref())
            .is_some(),
        true
    );
    assert_eq!(
        include
            .get(bag.get_type("articles").unwrap().as_ref())
            .is_some(),
        true
    );
}

#[test]
fn unknown_type() {
    let (bag, builder) = setup(r#"include=aaaaa"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(type_) if type_ == "aaaaa"),
        true,
        "wrong error type"
    );
}

#[test]
fn empty() {
    let (bag, builder) = setup(r#"include="#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(type_) if type_.is_empty()),
        true,
        "wrong error type"
    );
}

#[test]
fn unknown_second_relationships() {
    let (bag, builder) = setup(r#"include=peoples.aaaa"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(type_, type2) if type_ == "peoples" && type2 == "aaaa"),
        true,
        "wrong error type"
    );
}

#[test]
fn unknown_first_relationships() {
    let (bag, builder) = setup(r#"include=aaaa.peoples"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(type_) if type_ == "aaaa"),
        true,
        "wrong error type"
    );
}

#[test]
fn alias_type_with_nested_type() {
    let (bag, builder) = setup(r#"include=comments.author"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");

    let include = res.include();
    assert_eq!(include.len(), 1);
    assert_eq!(
        include
            .get(bag.get_type("peoples").unwrap().as_ref())
            .is_some(),
        true
    );
}
