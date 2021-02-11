use super::*;

#[test]
fn single() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=comments"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.map().get("comments").unwrap());
}

#[test]
fn multiple() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=comments,articles"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 2);
    assert_eq!(include[0], bag.map().get("comments").unwrap());
    assert_eq!(include[1], bag.map().get("articles").unwrap());
}

#[test]
fn single_with_nesting() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=peoples.comments"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.map().get("peoples").unwrap()))
        .expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.map().get("comments").unwrap());
}

#[test]
fn multiple_with_nesting() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=peoples.comments,peoples.articles"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.map().get("peoples").unwrap()))
        .expect("to build correctly");
    assert_eq!(res.include().is_some(), true);
    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 2);
    assert_eq!(include[0], bag.map().get("comments").unwrap());
    assert_eq!(include[1], bag.map().get("articles").unwrap());
}

#[test]
fn unknown_type() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=aaaaa"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

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
fn unknown_second_relationships() {
    let bag = gen_bag();

    const PARAM: &str = r#"include=peoples.aaaa"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

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
    let bag = gen_bag();

    const PARAM: &str = r#"include=aaaa.peoples"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

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
    let bag = gen_bag();

    const PARAM: &str = r#"include=comments.author"#;

    let builder: CibouletteQueryParametersBuilder =
        serde_urlencoded::from_str(PARAM).expect("to parse");

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");

    let include = res.include().as_ref().unwrap();
    assert_eq!(include.len(), 1);
    assert_eq!(include[0], bag.map().get("peoples").unwrap());
}
