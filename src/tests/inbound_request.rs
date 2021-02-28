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
    assert_eq!(res.query().is_none(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x == &store.get_type("articles").unwrap()),
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
    assert_eq!(res.query().is_none(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x == &store.get_type("comments").unwrap()),
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
    assert_eq!(res.query().is_some(), true);
    assert_eq!(
        matches!(res.path(), CiboulettePath::Type(x) if x == &store.get_type("comments").unwrap()),
        true
    );
}