use super::*;

#[test]
fn too_many_main_data() {
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
    let err = CibouletteResponseDataBuilder::new(
        &res,
        vec![
            gen_data_row(
                &store,
                &base_type,
                "comments",
                "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                true,
            ),
            gen_data_row(
                &store,
                &base_type,
                "comments",
                "68882db9-76e1-4139-a3a4-72c4fe6571f0",
                true,
            ),
        ],
    )
    .build(store.config())
    .unwrap_err();
    assert_eq!(
        matches!(err, CibouletteError::ResponseTooManyMainData(x) if x == "comments"),
        true
    );
}
