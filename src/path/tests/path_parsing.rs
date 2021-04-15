use super::*;

#[test]
fn single_type() {
    const VAL: &str = "/articles";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePathBuilder::Type(Cow::Borrowed(x)) if x == "articles"),
        true
    );
}

#[test]
fn no_type() {
    const VAL: &str = "/";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap_err(), CibouletteError::MissingTypeInPath),
        true
    );
}

#[test]
fn empty_segments() {
    const VAL: &str = "http://localhost///////peoples";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePathBuilder::Type(x) if x == "peoples"),
        true
    );
}

#[test]
fn too_much_type() {
    const VAL: &str =
        "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author/hello/world";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(matches!(res.unwrap_err(), CibouletteError::BadPath), true);
}

#[test]
fn bad_relationship_keyword() {
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relatioooooonships/author";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(matches!(res.unwrap_err(), CibouletteError::BadPath), true);
}

#[test]
fn type_id() {
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePathBuilder::TypeId(Cow::Borrowed(x), Cow::Borrowed(y)) if x == "articles" && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2"),
        true
    );
}

#[test]
fn type_id_relationship() {
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePathBuilder::TypeIdRelationship(Cow::Borrowed(x), Cow::Borrowed(y), Cow::Borrowed(z)) if x == "articles" && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2" && z == "author"),
        true
    );
}

#[test]
fn type_id_related() {
    const VAL: &str = "/articles/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/author";
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    let curr_url = opt.parse(VAL).unwrap();
    let res = CiboulettePathBuilder::parse(&curr_url);

    assert_eq!(
        matches!(res.unwrap(), CiboulettePathBuilder::TypeIdRelated(Cow::Borrowed(x), Cow::Borrowed(y), Cow::Borrowed(z)) if x == "articles" && y == "6720877a-e27e-4e9e-9ac0-3fff4deb55f2" && z == "author"),
        true
    );
}
