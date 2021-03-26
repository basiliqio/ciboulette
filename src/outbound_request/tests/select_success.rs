use super::*;

#[test]
fn simple_read_multi() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            "comments",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build()
    .unwrap();
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
    assert_json_snapshot!(response);
}

#[test]
fn simple_read_single() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/073b5936-0acb-4601-b4b7-9de607dfc2ef";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            "comments",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn simple_read_related() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/073b5936-0acb-4601-b4b7-9de607dfc2ef/articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
            ),
            gen_data_row(
                &store,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn simple_read_relationships() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/073b5936-0acb-4601-b4b7-9de607dfc2ef/relationships/articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                false,
            ),
            gen_data_row(
                &store,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                false,
            ),
            gen_data_row(
                &store,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                false,
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_included() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples?include=articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row_related(
                &store,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_multi_included() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples?include=articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
                true,
            ),
            gen_data_row_related(
                &store,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row_related(
                &store,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_multi_included_empty() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples?include=articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
                true,
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_single() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/073b5936-0acb-4601-b4b7-9de607dfc2ef?include=articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_single_empty() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/073b5936-0acb-4601-b4b7-9de607dfc2ef?include=articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            "peoples",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_related() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/c49094ce-71ab-40d4-a642-ea200f72eac6/articles?include=peoples";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let response = CibouletteOutboundRequestDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                "peoples",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build()
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}
