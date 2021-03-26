use super::*;

#[test]
fn simple_read() {
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
        vec![CibouletteResponseElement::new(
            &store,
            CibouletteResourceIdentifierBuilder::new(
                Some(CibouletteIdBuilder::Text(Cow::Borrowed(
                    "073b5936-0acb-4601-b4b7-9de607dfc2ef",
                ))),
                Cow::Borrowed("comments"),
            )
            .build_from_store(&store)
            .unwrap(),
            Some(String::from("<some data>")) as Option<String>,
            None,
        )
        .unwrap()],
    )
    .build()
    .unwrap();
    insta::assert_json_snapshot!(response);
}
