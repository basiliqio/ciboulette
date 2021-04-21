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
    assert_eq!(include[0][0].related_type().name(), "comments");
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
    assert_eq!(include[0][0].related_type().name(), "comments");
    assert_eq!(include[1][0].related_type().name(), "articles");
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
    assert_eq!(include[0][0].related_type().name(), "comments");
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
    assert_eq!(include[0][0].related_type().name(), "comments");
    assert_eq!(include[1][0].related_type().name(), "articles");
}

#[test]
fn unknown_type() {
    let (bag, builder) = setup(r#"include=aaaaa"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(type_, rel_name) if type_ == "peoples" && rel_name == "aaaaa"),
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
        matches!(err, CibouletteError::UnknownRelationship(type_, rel_name) if type_ == "peoples" && rel_name.is_empty()),
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
        matches!(err, CibouletteError::UnknownRelationship(type_, rel_name) if type_ == "peoples" && rel_name == "aaaa"),
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
        matches!(err, CibouletteError::UnknownRelationship(type_, rel_name) if type_ == "peoples" && rel_name == "aaaa"),
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
    assert_eq!(include[0][0].related_type().name(), "comments");
    assert_eq!(include[0][1].related_type().name(), "peoples");
}
