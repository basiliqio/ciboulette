use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteDeleteRequest<'request> {
    pub resource_type: Arc<CibouletteResourceType>,
    pub resource_id: CibouletteId<'request>,
    pub related_type: Option<Arc<CibouletteResourceType>>,
    pub query: CibouletteQueryParameters<'request>,
    pub meta: Option<Value>,
    pub expected_response_type: CibouletteResponseRequiredType,
    pub path: CiboulettePath<'request>,
}

impl<'request> CibouletteInboundRequestCommons<'request> for CibouletteDeleteRequest<'request> {
    fn path(&self) -> &CiboulettePath<'request> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Delete
    }

    fn expected_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().main_type()
    }

    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteInboundRequest<'request>> for CibouletteDeleteRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
            query,
            body,
            path,
            intention,
        } = value;

        let (resource_type, resource_id, related_type) = match &path {
            CiboulettePath::TypeId(type_, id) => (type_.clone(), id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => {
                (type_.clone(), id, Some(rel_type.clone()))
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
            related_type: related_type.map(|x| x.related_type().clone()),
            path,
            query,
            meta,
            expected_response_type: CibouletteResponseRequiredType::None,
        })
    }
}
