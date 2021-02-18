use super::*;

#[test]
fn ok() {
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(res.is_ok(), true);
    let res = res.unwrap();
    assert_eq!(res.resource_id(), "6720877a-e27e-4e9e-9ac0-3fff4deb55f2");
    assert_eq!(res.resource_type(), &store.get_type("comments").unwrap());
}

#[test]
fn without_id() {
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::MissingId)), true);
}

#[test]
fn wrong_path_type() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

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
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;
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
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(
        matches!(
            res,
            Err(CibouletteError::WrongIntention(x, y))
            if x == CibouletteIntention::Delete && y == CibouletteIntention::Update
        ),
        true
    );
}

#[test]
fn no_compound() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "comments",
				"attributes":
				{
					"body": "world"
				}
			},
			{
				"id": "9e5bd7e3-2304-40b1-9c7e-6ce9858322e4",
				"type": "comments",
				"attributes":
				{
					"body": "world"
				}
			}
		]
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::NoCompound)), true);
}
