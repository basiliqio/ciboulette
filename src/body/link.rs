use super::*;

/// ## A `json:api` inner [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'a> {
    pub href: Cow<'a, str>,
    pub meta: HashMap<Cow<'a, str>, Value>,
}

/// ## A selector between simple or complex `json:api` [link](https://jsonapi.org/format/#document-links) inner object
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CibouletteLinkSelector<'a> {
    Simple(Cow<'a, str>),
    Obj(CibouletteLinkObj<'a>),
}

/// ## A `json:api` [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteLink<'a> {
    #[serde(rename = "self")]
    pub self_: Option<CibouletteLinkSelector<'a>>,
    pub related: Option<CibouletteLinkSelector<'a>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteTopLevelPagination<'a> {
    pub first: Option<Cow<'a, str>>,
    pub last: Option<Cow<'a, str>>,
    pub prev: Option<Cow<'a, str>>,
    pub next: Option<Cow<'a, str>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteTopLevelLink<'a> {
    #[serde(flatten)]
    pub inner_link: CibouletteLink<'a>,
    #[serde(flatten)]
    pub pagination: CibouletteTopLevelPagination<'a>,
}
