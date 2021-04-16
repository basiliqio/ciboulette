use super::*;

#[test]
fn simple() {
    let (bag, builder) = setup(r#"filter=hello"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(res.filter().is_some(), true);
    let filter = res.filter().as_ref().unwrap();
    assert_eq!(filter, "hello");
}

#[test]
fn empty() {
    let (bag, builder) = setup(r#"filter="#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(res.filter().is_some(), true);
    let filter = res.filter().as_ref().unwrap();
    assert_eq!(filter, "");
}

#[test]
fn urlencoded() {
    let (bag, builder) = setup(r#"filter=%5Burlencoded%5D"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, bag.get_type("peoples").unwrap().clone())
        .expect("to build correctly");
    assert_eq!(res.filter().is_some(), true);
    let filter = res.filter().as_ref().unwrap();
    assert_eq!(filter, "[urlencoded]");
}
