use super::*;

#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'a> {
    href: Cow<'a, str>,
    meta: HashMap<Cow<'a, str>, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CibouletteLinkSelector<'a> {
    Simple(Cow<'a, str>),
    Obj(CibouletteLinkObj<'a>),
}

#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct CibouletteLink<'a> {
    #[serde(rename = "self")]
    self_: Option<CibouletteLinkSelector<'a>>,
    related: Option<CibouletteLinkSelector<'a>>,
}
