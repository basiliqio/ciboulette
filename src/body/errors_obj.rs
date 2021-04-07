use super::*;

/// ## A link describing a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorLink<'request> {
    pub about: Option<Cow<'request, str>>,
    #[serde(flatten)]
    pub inner_link: Option<CibouletteLinkObj<'request>>,
}

/// ## Source object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorSource<'request> {
    pub pointer: Option<Cow<'request, str>>,
    pub parameter: Option<Cow<'request, str>>,
    pub header: Option<Cow<'request, str>>,
}

/// ## Object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorObj<'request> {
    pub id: Option<Cow<'request, str>>,
    pub links: Option<CibouletteErrorLink<'request>>,
    pub status: u64,
    pub code: Option<Cow<'request, str>>,
    pub title: Option<Cow<'request, str>>,
    pub detail: Option<Cow<'request, str>>,
    pub source: Option<CibouletteErrorSource<'request>>,
}
