use super::*;

#[test]
fn delete() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/073b5936-0acb-4601-b4b7-9de607dfc2ef";
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request).unwrap();
    assert_eq!(
        matches!(
            crate::responses::links::build_link_for_response_root(store.config(), &res),
            None
        ),
        true
    );
    // assert_eq!(matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x == "/comments/073b5936-0acb-4601-b4b7-9de607dfc2ef"), true)
}

#[test]
fn delete_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80/".to_string());

    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/073b5936-0acb-4601-b4b7-9de607dfc2ef";
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request).unwrap();
    assert_eq!(
        matches!(
            crate::responses::links::build_link_for_response_root(store.config(), &res),
            None
        ),
        true
    );
}
