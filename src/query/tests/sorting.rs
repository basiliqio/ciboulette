use super::*;

#[test]
fn single_asc() {
    let (bag, builder) = setup(r#"sort=first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}

#[test]
fn single_asc_with_positive() {
    let (bag, builder) = setup(r#"sort=%2Bfirst-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}

#[test]
fn single_desc() {
    let (bag, builder) = setup(r#"sort=-first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Desc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}

#[test]
fn multiple_mixed() {
    let (bag, builder) = setup(r#"sort=last-name,-first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 2);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "last-name");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
    assert_eq!(
        matches!(sorting[1].direction(), &CibouletteSortingDirection::Desc),
        true
    );
    assert_eq!(sorting[1].field(), "first-name");
    assert_eq!(
        sorting[1].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}

#[test]
fn unknown_field() {
    let (bag, builder) = setup(r#"sort=unknown_field"#);

    let err: CibouletteError = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
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
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownField(type_, field) if type_.as_str() == "peoples" && field.as_str() == "<empty>"),
        true
    );
}

#[test]
fn relationship_field() {
    let (bag, builder) = setup(r#"sort=articles.title"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "title");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("articles").unwrap().as_ref()
    );
}

#[test]
fn mixed_relationship_and_self() {
    let (bag, builder) = setup(r#"sort=articles.title,-favorite_color.color,first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 3);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "title");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("articles").unwrap().as_ref()
    );

    assert_eq!(
        matches!(sorting[1].direction(), &CibouletteSortingDirection::Desc),
        true
    );
    assert_eq!(sorting[1].field(), "color");
    assert_eq!(
        sorting[1].type_().as_ref(),
        bag.get_type("favorite_color").unwrap().as_ref()
    );

    assert_eq!(
        matches!(sorting[2].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[2].field(), "first-name");
    assert_eq!(
        sorting[2].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}

#[test]
fn unknown_type() {
    let (bag, builder) = setup(r#"sort=hahahah.title"#);

    let err: CibouletteError = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(type_, rel) if type_.as_str() == "peoples" && rel.as_str() == "hahahah"),
        true
    );
}

#[test]
fn unknown_relationship_field() {
    let (bag, builder) = setup(r#"sort=articles.hahah"#);

    let err: CibouletteError = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownField(type_, field) if type_.as_str() == "articles" && field.as_str() == "hahah"),
        true
    );
}

#[test]
fn unknown_relationships() {
    let (bag, builder) = setup(r#"sort=favorite_color.color"#);

    let err: CibouletteError = builder
        .build(&bag, Some(bag.get_type("comments").unwrap().clone()))
        .expect_err("not to build correctly");
    assert_eq!(
        matches!(err, CibouletteError::UnknownRelationship(type_, rel) if type_.as_str() == "comments" && rel.as_str() == "favorite_color"),
        true
    );
}

#[test]
fn nested_sorting() {
    let (bag, builder) = setup(r#"sort=articles.comments.body"#);

    let err: CibouletteError = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect_err("not to build correctly");
    assert_eq!(matches!(err, CibouletteError::NestedSorting), true);
}

#[test]
fn sorting_self_full_path() {
    let (bag, builder) = setup(r#"sort=peoples.first-name"#);

    let res: CibouletteQueryParameters = builder
        .build(&bag, Some(bag.get_type("peoples").unwrap().clone()))
        .expect("to build correctly");
    let sorting = res.sorting();
    assert_eq!(sorting.len(), 1);
    assert_eq!(
        matches!(sorting[0].direction(), &CibouletteSortingDirection::Asc),
        true
    );
    assert_eq!(sorting[0].field(), "first-name");
    assert_eq!(
        sorting[0].type_().as_ref(),
        bag.get_type("peoples").unwrap().as_ref()
    );
}
