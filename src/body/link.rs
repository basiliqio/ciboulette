use super::*;

/// ## A `json:api` inner [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'request> {
    pub href: Cow<'request, str>,
    pub meta: HashMap<Cow<'request, str>, Value>,
}

/// ## A selector between simple or complex `json:api` [link](https://jsonapi.org/format/#document-links) inner object
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteLinkSelector<'request> {
    Simple(Cow<'request, str>),
    Obj(CibouletteLinkObj<'request>),
}

/// ## A `json:api` [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteLink<'request> {
    #[serde(rename = "self")]
    pub self_: Option<CibouletteLinkSelector<'request>>,
    pub related: Option<CibouletteLinkSelector<'request>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteBodyPagination<'request> {
    pub first: Option<Cow<'request, str>>,
    pub last: Option<Cow<'request, str>>,
    pub prev: Option<Cow<'request, str>>,
    pub next: Option<Cow<'request, str>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteBodyLink<'request> {
    #[serde(flatten)]
    pub inner_link: CibouletteLink<'request>,
    #[serde(flatten)]
    pub pagination: CibouletteBodyPagination<'request>,
}
