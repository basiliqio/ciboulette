use super::*;

#[test]
fn read_all_simple() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_one_simple() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_related_simple() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_relationship_simple() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/relationships/author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/relationships/author"),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author"),
        true
    );
}

#[test]
fn read_all_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_one_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_related_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn read_relationship_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/relationships/author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/relationships/author"),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments/b64686c7-5a9d-45b5-80d0-ac82d845d50f/author"),
        true
    );
}
