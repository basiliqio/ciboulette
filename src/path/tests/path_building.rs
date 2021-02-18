use super::*;

#[test]
fn single_type() {
    let store = gen_bag();
    const VAL: &str = "/articles";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePath::Type(x) if x == store.get_type("articles").unwrap()),
        true
    );
}

#[test]
fn unknown_first_type() {
    let store = gen_bag();
    const VAL: &str = "/AAAA";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap_err(), CibouletteError::UnknownType(x) if x == "AAAA"),
        true
    );
}

#[test]
fn type_id() {
    let store = gen_bag();
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePath::TypeId(x, Cow::Borrowed(y)) if x == store.get_type("articles").unwrap() && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2"),
        true
    );
}

#[test]
fn type_id_related() {
    let store = gen_bag();
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/author";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePath::TypeIdRelated(x, Cow::Borrowed(y), z) if x == store.get_type("articles").unwrap() && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2" && z == store.get_type("peoples").unwrap()),
        true
    );
}

#[test]
fn type_id_relationship() {
    let store = gen_bag();
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePath::TypeIdRelationship(x, Cow::Borrowed(y), z) if x == store.get_type("articles").unwrap() && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2" && z == store.get_type("peoples").unwrap()),
        true
    );
}

#[test]
fn type_id_relationship_unknown_last_type() {
    let store = gen_bag();
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/AAAA";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap_err(), CibouletteError::UnknownRelationship(x, y) if x == "articles" && y == "AAAA"),
        true
    );
}

#[test]
fn type_id_relationship_using_type_rather_than_alias() {
    let store = gen_bag();
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/peoples";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let builder = CiboulettePathBuilder::parse(&curr_url).unwrap();
    let res = builder.build(&store);

    assert_eq!(
        matches!(res.unwrap_err(), CibouletteError::UnknownRelationship(x, y) if x == "articles" && y == "peoples"),
        true
    );
}
