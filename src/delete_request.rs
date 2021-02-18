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
        let CibouletteRequest {
            query,
            body,
            path,
            intention,
        } = value;

        let path_type = CiboulettePathType::from(&path);

        if !matches!(path_type, CiboulettePathType::TypeId) {
            return Err(CibouletteError::WrongPathType(
                path_type,
                vec![CiboulettePathType::TypeId],
            ));
        }

        if !matches!(intention, CibouletteIntention::Delete) {
            return Err(CibouletteError::WrongIntention(
                intention,
                CibouletteIntention::Delete,
            ));
        }

        let CibouletteBody { meta, .. } = body.unwrap_or_default();
        Ok(CibouletteDeleteRequest {
            query: query.unwrap_or_default(),
            meta,
        })
    }
}
