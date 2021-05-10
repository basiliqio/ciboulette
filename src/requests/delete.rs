use super::*;

/// ## A `DELETE` request
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteDeleteRequest<'request> {
    /// The resource type beeing deleted
    pub resource_type: Arc<CibouletteResourceType>,
    /// The resource id beeing delete
    pub resource_id: CibouletteIdSelector<'request>,
    /// The related type beeing delete when deleting M2O relationships
    pub related_type: Option<Arc<CibouletteResourceType>>,
    /// The query parameters
    pub query: CibouletteQueryParameters<'request>,
    /// The meta data sent by the client
    pub meta: Option<Value>,
    /// The expected response type
    pub expected_response_type: CibouletteResponseRequiredType,
    /// The path used to query
    pub path: CiboulettePath<'request>,
}

impl<'request> CibouletteRequestCommons<'request> for CibouletteDeleteRequest<'request> {
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

    fn anchor_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().main_type()
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteRequest<'request>> for CibouletteDeleteRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'request>) -> Result<Self, Self::Error> {
        let CibouletteRequest {
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
