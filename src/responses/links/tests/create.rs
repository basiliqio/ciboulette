use super::*;

#[test]
fn create_simple() {
    let store = gen_bag();
    // *store.config_mut().base_url_mut() = Some("http://localhost:80/".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body: String = json!({
        "data": json!({
            "type": "comments",
            "attributes": json!({
                "body": "Hello World"
            })
        })
    })
    .to_string();
    let body_str = Some(body.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body_str);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn create_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body: String = json!({
        "data": json!({
            "type": "comments",
            "attributes": json!({
                "body": "Hello World"
            })
        })
    })
    .to_string();
    let body_str = Some(body.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body_str);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}
