use super::*;
/// An outbound response, built from an inbound request.
#[derive(Debug, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteErrorRequest<'response> {
    /// The body of the response.
    pub errors: CibouletteErrorObj<'response>,
    /// The status of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}

impl<'response> CibouletteErrorRequest<'response> {
    pub fn new(errors: CibouletteErrorObj<'response>, meta: Option<Value>) -> Self {
        CibouletteErrorRequest { errors, meta }
    }
}
