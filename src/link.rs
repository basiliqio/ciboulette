use super::*;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'a> {
    href: Cow<'a, str>,
    meta: HashMap<Cow<'a, str>, Value>,
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CibouletteLink<'a> {
    self_: Option<Either<Cow<'a, str>, CibouletteLinkObj<'a>>>,
    related: Option<Either<Cow<'a, str>, CibouletteLinkObj<'a>>>,
}
