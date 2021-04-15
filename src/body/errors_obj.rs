use super::*;

/// ## A link describing a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone, Default)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorLink<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<Cow<'request, str>>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_link: Option<CibouletteLinkObj<'request>>,
}

/// ## Source object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone, Default)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorSource<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Cow<'request, str>>,
}

/// ## Object of a `json:api` [error](https://jsonapi.org/format/#error-objects)
#[derive(Debug, Deserialize, Serialize, Getters, MutGetters, Clone, Default)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorObj<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteErrorLink<'request>>,
    pub status: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CibouletteErrorSource<'request>>,
}
