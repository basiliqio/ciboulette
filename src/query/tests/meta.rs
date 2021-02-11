use super::*;

#[test]
fn simple() {
    let (bag, builder) = setup(r#"hahaha=hohoho"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.meta().len(), 1);
    let meta = res.meta().get("hahaha").unwrap();
    assert_eq!(meta, "hohoho");
}

#[test]
fn empty_keys() {
    let (bag, builder) = setup(r#"=hohoho"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.meta().len(), 1);
    let meta = res.meta().get("").unwrap();
    assert_eq!(meta, "hohoho");
}

#[test]
fn empty_value() {
    let (bag, builder) = setup(r#"hahaha="#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.meta().len(), 1);
    let meta = res.meta().get("hahaha").unwrap();
    assert_eq!(meta, "");
}

#[test]
fn urlencoded() {
    let (bag, builder) = setup(r#"%5Bhahaha%5D=%5Bhohoho%5D"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    assert_eq!(res.meta().len(), 1);
    let meta = res.meta().get("[hahaha]").unwrap();
    assert_eq!(meta, "[hohoho]");
}
