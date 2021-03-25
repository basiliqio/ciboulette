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
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

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
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
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
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
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
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
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
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::NoCompound)), true);
}

#[test]
fn relationship_null() {
    let bag = gen_bag();
    const VAL: &str = r#"
	{
		"data": {
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "world",
				"last-name": "hello"
			},
			"relationships":
			{
				"favorite_color":
				{
					"data": null
				}
			}
		},
		"meta":
		{
			"self": "peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2"
		}
	}
	"#;
    let uri = Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    let doc = CibouletteInboundRequestBuilder::new(CibouletteIntention::Update, &uri, &Some(VAL))
        .build(&bag)
        .expect("to build the document");
    let req = CibouletteUpdateRequest::try_from(doc).unwrap();
    if let CibouletteUpdateRequestType::MainType(obj) = req.data() {
        let v = obj.relationships().get("favorite_color").unwrap();
        assert_eq!(
            matches!(v.data(), CibouletteOptionalData::Null(x) if *x),
            true
        );
    } else {
        panic!();
    }
}

#[test]
fn main_key_clash_main() {
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
			"type": "peoples",
			"attributes":
			{
				"first-name": "world"
			}
		}
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = builder.build(&store).unwrap();
    let res = CibouletteUpdateRequest::try_from(request);

    assert_eq!(matches!(res, Err(CibouletteError::MainTypeClash)), true);
}
