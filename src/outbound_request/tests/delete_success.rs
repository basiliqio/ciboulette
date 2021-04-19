use super::*;

#[test]
fn simple_update() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/073b5936-0acb-4601-b4b7-9de607dfc2ef";
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::<'_, '_, String, _>::new(&res, vec![])
        .build(&store)
        .unwrap();
    assert_eq!(response.status(), CibouletteResponseStatus::OkEmpty);
    assert_json_snapshot!(response);
}
