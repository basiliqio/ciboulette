use super::*;

#[test]
fn simple_create() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body_str: String = json!({
        "data": json!({
            "type": "comments",
            "attributes": json!({
                "body": "Hello World"
            })
        })
    })
    .to_string();
    let body: Option<&str> = Some(body_str.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            "comments",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build(&store)
    .unwrap();
    assert_eq!(response.status(), CibouletteResponseStatus::Created);
    assert_json_snapshot!(response);
}

#[test]
fn simple_create_no_content() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body_str: String = json!({
        "data": json!({
            "type": "comments",
            "attributes": json!({
                "body": "Hello World"
            })
        })
    })
    .to_string();
    let body: Option<&str> = Some(body_str.as_str());

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::<'_, '_, String, _>::new(&res, vec![])
        .build(&store)
        .unwrap();
    assert_eq!(response.status(), CibouletteResponseStatus::OkEmpty);
    assert_json_snapshot!(response);
}
