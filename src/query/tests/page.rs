use super::*;
use std::borrow::Cow;

#[test]
fn simple_number() {
    let (bag, builder) = setup(r#"page[number]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Number).unwrap();
    assert_eq!(page, "1");
}

#[test]
fn simple_size() {
    let (bag, builder) = setup(r#"page[size]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Size).unwrap();
    assert_eq!(page, "1");
}

#[test]
fn simple_offset() {
    let (bag, builder) = setup(r#"page[offset]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Offset).unwrap();
    assert_eq!(page, "1");
}

#[test]
fn simple_limit() {
    let (bag, builder) = setup(r#"page[limit]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Limit).unwrap();
    assert_eq!(page, "1");
}

#[test]
fn simple_cursor() {
    let (bag, builder) = setup(r#"page[cursor]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Cursor).unwrap();
    assert_eq!(page, "1");
}

#[test]
fn simple_other() {
    let (bag, builder) = setup(r#"page[lolilol]=1"#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page
        .get(&CiboulettePageType::Other(Cow::Borrowed("lolilol")))
        .unwrap();
    assert_eq!(page, "1");
}

#[test]
fn empty_value() {
    let (bag, builder) = setup(r#"page[cursor]="#);

    let res: CibouletteQueryParameters = builder.build(&bag, None).expect("to build correctly");
    let page = res.page();
    assert_eq!(page.len(), 1);
    let page = page.get(&CiboulettePageType::Cursor).unwrap();
    assert_eq!(page, "");
}
