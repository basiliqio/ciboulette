use super::*;

#[derive(Deserialize, Serialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'a> {
    href: Cow<'a, str>,
    rel: Option<Cow<'a, str>>,
    described_by: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    type_: Option<Cow<'a, str>>,
    hreflang: Option<Cow<'a, str>>,
    meta: HashMap<Cow<'a, str>, Value>,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct CibouletteLink<'a> {
    self_: Option<Either<Cow<'a, str>, CibouletteLinkObj<'a>>>,
    related: Option<Either<Cow<'a, str>, CibouletteLinkObj<'a>>>,
}
