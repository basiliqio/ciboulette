use super::*;

#[test]
fn ok() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
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
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(res.is_ok(), true);
}

#[test]
fn ok_with_id() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
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
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(res.is_ok(), true);
}

#[test]
fn wrong_path_type() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
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
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(
        matches!(
            res,
            Err(CibouletteError::WrongPathType(
                x,
                y
            )) if x == CiboulettePathType::TypeId && y == vec![CiboulettePathType::Type]
        ),
        true
    );
}

#[test]
fn wrong_intention() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;
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
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(
        matches!(
            res,
            Err(CibouletteError::WrongIntention(
                x,
                y
            )) if x == CibouletteIntention::Read && y == CibouletteIntention::Create
        ),
        true
    );
}

#[test]
fn no_body() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::NoData)), true);
}

#[test]
fn no_data() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"meta":
		{
			"hello_world": "/comments"
		}
	}
	"#,
    );
    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::NoData)), true);
}

#[test]
fn no_compound() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		[
			{
				"type": "comments",
				"attributes":
				{
					"body": "world"
				}
			},
			{
				"type": "comments",
				"attributes":
				{
					"body": "world2"
				}
			}
		]
	}
	"#,
    );
    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::NoCompound)), true);
}

#[test]
fn main_type_clash() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			}
		}
	}
	"#,
    );
    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteCreateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::MainTypeClash)), true);
}
