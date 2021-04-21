use super::*;

#[test]
fn simple_type() {
    let (bag, builder) = setup(r#"fields[peoples]=first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sparse = res.sparse();
    assert_eq!(sparse.len(), 1);
    let sparse = sparse
        .get(bag.get_type("peoples").unwrap().as_ref())
        .unwrap();
    assert_eq!(sparse.len(), 1);
    assert_eq!(sparse[0], "first-name");
}

#[test]
fn multiple_fields() {
    let (bag, builder) = setup(r#"fields[peoples]=first-name,last-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sparse = res.sparse();
    assert_eq!(sparse.len(), 1);
    let sparse = sparse
        .get(bag.get_type("peoples").unwrap().as_ref())
        .unwrap();
    assert_eq!(sparse.len(), 2);
    assert_eq!(sparse[0], "first-name");
    assert_eq!(sparse[1], "last-name");
}

#[test]
fn multiple_types() {
    let (bag, builder) = setup(r#"fields[peoples]=first-name&fields[articles]=title"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sparse = res.sparse();
    assert_eq!(sparse.len(), 2);
    let peoples = sparse
        .get(bag.get_type("peoples").unwrap().as_ref())
        .unwrap();
    assert_eq!(peoples.len(), 1);
    assert_eq!(peoples[0], "first-name");
    let articles = sparse
        .get(bag.get_type("articles").unwrap().as_ref())
        .unwrap();
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0], "title");
}

#[test]
fn unknown_type() {
    let (bag, builder) = setup(r#"fields[AAAA]=first-name"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(type_) if type_.as_str() == "AAAA"),
        true
    );
}

#[test]
fn unknown_fields() {
    let (bag, builder) = setup(r#"fields[peoples]=AAAA"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownField(type_, field) if type_.as_str() == "peoples" && field.as_str() == "AAAA"),
        true
    );
}

#[test]
fn empty_type() {
    let (bag, builder) = setup(r#"fields[]=AAAA"#);

    let err: CibouletteError = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownType(type_) if type_.is_empty()),
        true
    );
}

#[test]
fn empty_field() {
    let (bag, builder) = setup(r#"fields[peoples]="#);

    let query: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to_build correctly");
    let peoples_sparse = query
        .sparse()
        .get(bag.get_type("peoples").unwrap().as_ref())
        .unwrap();
    assert_eq!(peoples_sparse.len(), 0);
}

#[test]
fn url_encoded_type() {
    let (bag, builder) = setup(r#"fields%5Bpeoples%5D=first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    let sparse = res.sparse();
    assert_eq!(sparse.len(), 1);
    let sparse = sparse
        .get(bag.get_type("peoples").unwrap().as_ref())
        .unwrap();
    assert_eq!(sparse.len(), 1);
    assert_eq!(sparse[0], "first-name");
}
