use super::*;

/// ## A link describing a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorLink<'a> {
    pub about: Option<Cow<'a, str>>,
    #[serde(flatten)]
    pub inner_link: Option<CibouletteLinkObj<'a>>,
}

/// ## Source object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorSource<'a> {
    pub pointer: Option<Cow<'a, str>>,
    pub parameter: Option<Cow<'a, str>>,
    pub header: Option<Cow<'a, str>>,
}

/// ## Object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorObj<'a> {
    pub id: Option<Cow<'a, str>>,
    pub links: Option<CibouletteErrorLink<'a>>,
    pub status: u64,
    pub code: Option<Cow<'a, str>>,
    pub title: Option<Cow<'a, str>>,
    pub detail: Option<Cow<'a, str>>,
    pub source: Option<CibouletteErrorSource<'a>>,
}
