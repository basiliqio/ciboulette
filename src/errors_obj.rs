use super::*;

#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorLink<'a> {
    about: Option<Cow<'a, str>>,
    // flatten links
}

#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorSource<'a> {
    pointer: Option<Cow<'a, str>>,
    parameter: Option<Cow<'a, str>>,
    header: Option<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteErrorObj<'a> {
    id: Option<Cow<'a, str>>,
    links: Option<CibouletteErrorLink<'a>>,
    status: u64,
    code: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    detail: Option<Cow<'a, str>>,
    source: Option<CibouletteErrorSource<'a>>,
}
