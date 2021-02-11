use super::*;

#[test]
fn simple() {
    let (bag, builder) = setup(r#"filter[hola]=hello"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.filter_typed().len(), 1);
    let filter = res.filter_typed().get("hola").unwrap();
    assert_eq!(filter, "hello");
}

#[test]
fn empty_type() {
    let (bag, builder) = setup(r#"filter[]=hello"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.filter_typed().len(), 1);
    let filter = res.filter_typed().get("").unwrap();
    assert_eq!(filter, "hello");
}

#[test]
fn url_encoded() {
    let (bag, builder) = setup(r#"filter%5Bhola%5D=hel%2Blo"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.filter_typed().len(), 1);
    let filter = res.filter_typed().get("hola").unwrap();
    assert_eq!(filter, "hel+lo");
}

#[test]
fn empty_value() {
    let (bag, builder) = setup(r#"filter[hola]="#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.filter_typed().len(), 1);
    let filter = res.filter_typed().get("hola").unwrap();
    assert_eq!(filter, "");
}
