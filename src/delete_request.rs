use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteDeleteRequest<'a> {
    pub query: CibouletteQueryParameters<'a>,
    pub meta: Value,
}

impl<'a> TryFrom<CibouletteRequest<'a>> for CibouletteDeleteRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteRequest { query, body, .. } = value;
        let CibouletteBody { meta, .. } = body.unwrap_or_default();
        Ok(CibouletteDeleteRequest {
            query: query.unwrap_or_default(),
            meta,
        })
    }
}
