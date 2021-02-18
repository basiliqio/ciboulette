use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteDeleteRequest<'a> {
    pub resource_type: &'a CibouletteResourceType,
    pub resource_id: Cow<'a, str>,
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

        let (resource_type, resource_id) = match path {
            CiboulettePath::TypeId(type_, id) => (type_, id),
            _ => {
                return Err(CibouletteError::WrongPathType(
                    CiboulettePathType::from(&path),
                    vec![CiboulettePathType::TypeId],
                ))
            }
        };

        if !matches!(intention, CibouletteIntention::Delete) {
            return Err(CibouletteError::WrongIntention(
                intention,
                CibouletteIntention::Delete,
            ));
        }

        let CibouletteBody { meta, .. } = body.unwrap_or_default();
        Ok(CibouletteDeleteRequest {
            resource_type,
            resource_id,
            query: query.unwrap_or_default(),
            meta,
        })
    }
}
