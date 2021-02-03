use super::*;

#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[serde(rename = "camelCase")]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteTopLevel<'a> {
    // data: Option<CibouletteResourceSelector<'a>>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Option<Value>,
}
