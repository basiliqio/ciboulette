use super::*;

/// ## A 'GET' request
#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct CibouletteReadRequest<'request> {
    /// The path used to query
    pub path: CiboulettePath<'request>,
    /// The query parameters used
    pub query: CibouletteQueryParameters<'request>,
    /// The included data if any
    pub data: CibouletteResourceSelector<
        'request,
        MessyJsonObjectValue<'request>,
        CibouletteResourceIdentifier<'request>,
    >,
    /// The meta data sent by the client
    pub meta: Option<Value>,
    /// The expected response type
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request> CibouletteRequestCommons<'request> for CibouletteReadRequest<'request> {
    fn path(&self) -> &CiboulettePath<'request> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request> {
        &self.query
    }

    fn expected_type(&self) -> &Arc<CibouletteResourceType> {
        // match self.path() {
        //     CiboulettePath::TypeIdRelationship(_, _, _) => self.path().base_type(),
        //     _ => self.path().main_type(),
        // }
        self.path().main_type()
    }

    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Read
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn anchor_type(&self) -> &Arc<CibouletteResourceType> {
        match self.path() {
            CiboulettePath::TypeIdRelationship(_, _, _) => self.path().base_type(),
            _ => self.path().main_type(),
        }
        // self.path().main_type()
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteRequest<'request>> for CibouletteReadRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'request>) -> Result<Self, Self::Error> {
        let CibouletteRequest {
            query,
            body,
            intention,
            path,
        } = value;
        let expected_response_type: CibouletteResponseRequiredType = match path {
            CiboulettePath::TypeId(_, _) => {
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Single)
            }
            CiboulettePath::Type(_) | CiboulettePath::TypeIdRelated(_, _, _) => {
                CibouletteResponseRequiredType::Object(CibouletteResponseQuantity::Multiple)
            }
            CiboulettePath::TypeIdRelationship(_, _, _) => {
                CibouletteResponseRequiredType::Id(CibouletteResponseQuantity::Multiple)
            }
        };
        if !matches!(intention, CibouletteIntention::Read) {
            return Err(CibouletteError::WrongIntention(
                intention,
                CibouletteIntention::Read,
            ));
        }

        let CibouletteBody { data, meta, .. } = body.unwrap_or_default();

        let data = match data {
            CibouletteBodyData::Object(obj) => obj,
            CibouletteBodyData::Null(_) => CibouletteResourceSelector::<
                MessyJsonObjectValue<'request>,
                CibouletteResourceIdentifierPermissive<'_>,
            >::new(CibouletteSelector::Multi(Vec::new())),
        }
        .try_into()?;
        Ok(CibouletteReadRequest {
            path,
            query,
            data,
            meta,
            expected_response_type,
        })
    }
}
