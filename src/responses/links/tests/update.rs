use super::*;

#[test]
fn update_simple() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/a917bd07-29f6-427e-89ca-399a3768da87";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: String = json!({
        "data": json!({
            "type": "comments",
            "id": "a917bd07-29f6-427e-89ca-399a3768da87",
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
    let res = CibouletteUpdateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/comments/a917bd07-29f6-427e-89ca-399a3768da87"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn update_simple_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/a917bd07-29f6-427e-89ca-399a3768da87";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: String = json!({
        "data": json!({
            "type": "comments",
            "id": "a917bd07-29f6-427e-89ca-399a3768da87",
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
    let res = CibouletteUpdateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/comments/a917bd07-29f6-427e-89ca-399a3768da87"),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn update_rel() {
    let store = gen_bag();

    let url = Url::parse("http://localhost/peoples/0c7b2dc3-1a33-453c-9728-b53dac7ec46c/relationships/favorite_color").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/relationships/favorite_color";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: String = json!({
        "data": json!({
            "type": "favorite_color",
            "id": "3fec6234-717b-4a93-8977-992168be747c",
            "attributes": json!({
                "color": "red"
            })
        })
    })
    .to_string();
    let body_str = Some(body.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body_str);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/relationships/favorite_color"),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/favorite_color"),
        true
    );
}

#[test]
fn update_rel_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let url = Url::parse("http://localhost/peoples/0c7b2dc3-1a33-453c-9728-b53dac7ec46c/relationships/favorite_color").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/relationships/favorite_color";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: String = json!({
        "data": json!({
            "type": "favorite_color",
            "id": "3fec6234-717b-4a93-8977-992168be747c",
            "attributes": json!({
                "color": "red"
            })
        })
    })
    .to_string();
    let body_str = Some(body.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body_str);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request).unwrap();
    let link = crate::responses::links::build_link_for_response_root(store.config(), &res).unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/relationships/favorite_color"),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == "http://localhost:80/peoples/616aca3f-b86e-4aef-ac91-49644f057f06/favorite_color"),
        true
    );
}
