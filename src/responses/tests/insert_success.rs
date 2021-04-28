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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let base_type = store.get_type("comments").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            &base_type,
            "comments",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build()
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request).unwrap();
    let response = CibouletteResponseDataBuilder::<'_, '_, String, _>::new(&res, vec![])
        .build()
        .unwrap();
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
    assert_json_snapshot!(response);
}
