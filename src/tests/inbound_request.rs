use super::*;

#[test]
fn ok_path_only() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/articles";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let res = builder.build(&store).unwrap();

    assert_eq!(res.intention(), &INTENTION);
    assert_eq!(res.body().is_none(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x.as_ref() == store.get_type("articles").unwrap().as_ref()),
        true
    );
}

#[test]
fn body() {
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
    let res = builder.build(&store).unwrap();

    assert_eq!(res.intention(), &INTENTION);
    assert_eq!(res.body().is_some(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x.as_ref() == store.get_type("comments").unwrap().as_ref()),
        true
    );
}

#[test]
fn query() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/comments?sort=body";
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let res = builder.build(&store).unwrap();

    assert_eq!(res.intention(), &INTENTION);
    assert_eq!(res.body().is_none(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x.as_ref() == store.get_type("comments").unwrap().as_ref()),
        true
    );
}

#[test]
fn force_mandatory_to_null() {
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
				"body": null
			}
		}
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    builder.build(&store).unwrap_err();
}

#[test]
fn multi_id_in_query() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str =
        "/people-article/6720877a-e27e-4e9e-9ac0-3fff4deb55f2,80cb6984-87e4-4e22-ac04-008fc1ffca11";
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;
    const BODY: Option<&str> = None;

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let res = builder.build(&store).unwrap();

    assert_eq!(res.intention(), &INTENTION);
    assert_eq!(
        matches!(res.path(), CiboulettePath::TypeId(_, x) if x.len() == 2),
        true
    );
}

#[test]
fn multi_id_in_body() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str =
        "/people-article/6720877a-e27e-4e9e-9ac0-3fff4deb55f2,80cb6984-87e4-4e22-ac04-008fc1ffca11";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2,80cb6984-87e4-4e22-ac04-008fc1ffca11",
			"type": "people-article",
			"attributes":
			{
				"people_id": "158345cf-4e1a-4f74-a1e5-093d40d182a0",
				"article_id": "e152cda9-e24f-4c0c-94b5-17158f75049f"
			}
		}
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let res = builder.build(&store).unwrap();

    assert_eq!(res.intention(), &INTENTION);
    assert_eq!(
        matches!(res.path(), CiboulettePath::TypeId(_, x) if x.len() == 2),
        true
    );
}

#[test]
fn multi_id_in_relationship() {
    let store = gen_bag();
    let url = Url::parse("http://localhost/").unwrap();
    let opt = url::Url::options().base_url(Some(&url));
    const URL: &str = "/peoples/6dd511c2-cefd-4352-a70e-e9e8fe338fbe";
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6dd511c2-cefd-4352-a70e-e9e8fe338fbe",
			"type": "peoples",
			"attributes":
			{
				"first-name": "AAAAAAAAAAAAAAAAAAA"
			},
			"relationships":
			{
				"people-article": {
					"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2,80cb6984-87e4-4e22-ac04-008fc1ffca11",
					"type": "people-article"
				}
			}
		}
	}
	"#,
    );

    let parsed_url = opt.parse(URL).unwrap();
    let builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    builder.build(&store).unwrap();
}
