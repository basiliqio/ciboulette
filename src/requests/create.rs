use super::*;

/// ## A `POST` request
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteCreateRequest<'request> {
    /// The path of the request
    pub path: CiboulettePath<'request>,
    /// The query used
    pub query: CibouletteQueryParameters<'request>,
    /// The data sent by the client
    pub data: CibouletteResource<
        'request,
        MessyJsonObjectValue<'request>,
        CibouletteResourceIdentifierPermissive<'request>,
    >,
    /// The meta data sent by the client
    pub meta: Option<Value>,
    /// What response type to expect from that request.
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request> CibouletteRequestCommons<'request> for CibouletteCreateRequest<'request> {
    fn path(&self) -> &CiboulettePath<'request> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Create
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn expected_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().main_type()
    }

    fn anchor_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().main_type()
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteRequest<'request>> for CibouletteCreateRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'request>) -> Result<Self, Self::Error> {
        let CibouletteRequest {
            query,
            body,
            intention,
            path,
        } = value;

        let path_type = CiboulettePathType::from(&path);

        if !matches!(path_type, CiboulettePathType::Type) {
            return Err(CibouletteError::WrongPathType(
                path_type,
                vec![CiboulettePathType::Type],
            ));
        }

        if !matches!(intention, CibouletteIntention::Create) {
            return Err(CibouletteError::WrongIntention(
                intention,
                CibouletteIntention::Create,
            ));
        }

        let CibouletteBody { data, meta, .. } = body.ok_or(CibouletteError::NoData)?;

        let data = match data {
            CibouletteBodyData::Object(x) => x,
            CibouletteBodyData::Null(_) => return Err(CibouletteError::NoData),
        };
        let data = match data.take() {
            CibouletteSelector::Single(data) => {
                if data.identifier().type_() != path.main_type().name() {
                    return Err(CibouletteError::MainTypeClash);
                }
                data
            }
            _ => return Err(CibouletteError::NoCompound),
        };

        Ok(CibouletteCreateRequest {
            path,
            query,
            data,
            meta,
            expected_response_type: CibouletteResponseRequiredType::Object(
                CibouletteResponseQuantity::Single,
            ),
        })
    }
}
