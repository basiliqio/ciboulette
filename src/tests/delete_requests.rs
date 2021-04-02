use super::*;

#[test]
fn ok() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request);

    assert_eq!(res.is_ok(), true);

    let res = res.unwrap();
    assert_eq!(
        res.resource_id(),
        &CibouletteId::parse(
            CibouletteIdType::Uuid,
            Cow::Borrowed("6720877a-e27e-4e9e-9ac0-3fff4deb55f2")
        )
        .unwrap()
    );
    assert_eq!(
        res.resource_type(),
        &store.get_type("comments").unwrap().as_ref()
    );
}

#[test]
fn wrong_path_type() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request);

    assert_eq!(
        matches!(
            res,
            Err(CibouletteError::WrongPathType(x, y))
            if x == CiboulettePathType::Type && y == vec![CiboulettePathType::TypeId]
        ),
        true
    );
}

#[test]
fn wrong_intention() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "comments",
			"attributes":
			{
				"body": "world"
			}
		}
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteDeleteRequest::try_from(request);

    assert_eq!(
        matches!(
            res,
            Err(CibouletteError::WrongIntention(x, y))
            if x == CibouletteIntention::Update && y == CibouletteIntention::Delete
        ),
        true
    );
}
