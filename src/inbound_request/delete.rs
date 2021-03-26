use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteDeleteRequest<'a> {
    pub resource_type: &'a CibouletteResourceType<'a>,
    pub resource_id: CibouletteId<'a>,
    pub related_type: Option<&'a CibouletteResourceType<'a>>,
    pub query: CibouletteQueryParameters<'a>,
    pub meta: Value,
    pub expected_response_type: CibouletteResponseRequiredType,
    pub path: CiboulettePath<'a>,
}

impl<'a> CibouletteInboundRequestCommons<'a> for CibouletteDeleteRequest<'a> {
    fn path(&self) -> &CiboulettePath<'a> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'a> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Delete
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &serde_json::Value {
        &self.meta
    }
}

impl<'a> TryFrom<CibouletteInboundRequest<'a>> for CibouletteDeleteRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
            query,
            body,
            path,
            intention,
        } = value;

        let (resource_type, resource_id, related_type) = match &path {
            CiboulettePath::TypeId(type_, id) => (*type_, id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => {
                (*type_, id, Some(*rel_type))
            }
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
            resource_id: resource_id.clone(),
            related_type,
            path,
            query,
            meta,
            expected_response_type: CibouletteResponseRequiredType::None,
        })
    }
}
