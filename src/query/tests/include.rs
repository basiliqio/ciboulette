use super::*;

#[test]
fn single() {
    let (bag, builder) = setup(r#"include=comments"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.get_type("comments").unwrap());
}

#[test]
fn multiple() {
    let (bag, builder) = setup(r#"include=comments,articles"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 2);
    assert_eq!(include[0], bag.get_type("comments").unwrap());
    assert_eq!(include[1], bag.get_type("articles").unwrap());
}

#[test]
fn single_with_nesting() {
    let (bag, builder) = setup(r#"include=peoples.comments"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap()))
        .expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.get_type("comments").unwrap());
}

#[test]
fn multiple_with_nesting() {
    let (bag, builder) = setup(r#"include=peoples.comments,peoples.articles"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap()))
        .expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 2);
    assert_eq!(include[0], bag.get_type("comments").unwrap());
    assert_eq!(include[1], bag.get_type("articles").unwrap());
}

#[test]
fn unknown_type() {
    let (bag, builder) = setup(r#"include=aaaaa"#);

    let err: CibouletteError = builder
        .build(&bag, None)
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
        .build(&bag, None)
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
        .build(&bag, None)
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
        .build(&bag, None)
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

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");

    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.get_type("peoples").unwrap());
}
