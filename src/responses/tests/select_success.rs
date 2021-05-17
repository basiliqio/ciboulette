use super::*;

#[test]
fn simple_read_multi() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
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
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
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
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
            ),
            gen_data_row(
                &store,
                &base_type,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "ea2c0ae1-e3b0-41b2-82c2-2f000d4dc367",
                false,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                false,
                "peoples",
                "ea2c0ae1-e3b0-41b2-82c2-2f000d4dc367",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                false,
                "peoples",
                "ea2c0ae1-e3b0-41b2-82c2-2f000d4dc367",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                false,
                "peoples",
                "ea2c0ae1-e3b0-41b2-82c2-2f000d4dc367",
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
                true,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "90e676f5-9598-44e7-9e5b-f0c7b1a188d1",
                true,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "01b8fb1f-989b-4ec0-86a0-72786017682b",
                true,
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build(store.config())
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![gen_data_row(
            &store,
            &base_type,
            "peoples",
            "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            true,
        )],
    )
    .build(store.config())
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_related() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/c49094ce-71ab-40d4-a642-ea200f72eac6/articles?include=author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("articles").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "author",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "articles",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
        ],
    )
    .build(store.config())
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_nested() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/c49094ce-71ab-40d4-a642-ea200f72eac6?include=articles.comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles.comments",
                "f2dde16b-d26d-4b34-944f-7430c81e2d8e",
                true,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
            ),
        ],
    )
    .build(store.config())
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}

#[test]
fn with_include_nested_self() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str =
        "/peoples/c49094ce-71ab-40d4-a642-ea200f72eac6?include=articles.comments.author";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = builder.build(&store).unwrap();
    let res = CibouletteReadRequest::try_from(request).unwrap();
    let base_type = store.get_type("peoples").unwrap();
    let response = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row_related(
                &store,
                &base_type,
                "articles.comments",
                "f2dde16b-d26d-4b34-944f-7430c81e2d8e",
                true,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles.comments.author",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
                "comments",
                "f2dde16b-d26d-4b34-944f-7430c81e2d8e",
            ),
            gen_data_row_related(
                &store,
                &base_type,
                "articles",
                "c49094ce-71ab-40d4-a642-ea200f72eac6",
                true,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
            ),
            gen_data_row(
                &store,
                &base_type,
                "peoples",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
        ],
    )
    .build(store.config())
    .unwrap();
    assert_json_snapshot!(response);
    assert_eq!(response.status(), CibouletteResponseStatus::Ok);
}
