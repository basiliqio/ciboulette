use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteCreateRequest<'request> {
    pub path: CiboulettePath<'request>,
    pub query: CibouletteQueryParameters<'request>,
    pub data: CibouletteResource<
        'request,
        MessyJsonObjectValue<'request>,
        CibouletteResourceIdentifierPermissive<'request>,
    >,
    pub meta: Option<Value>,
    pub links: Option<CibouletteBodyLink<'request>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>,
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request> CibouletteInboundRequestCommons<'request> for CibouletteCreateRequest<'request> {
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

impl<'request> TryFrom<CibouletteInboundRequest<'request>> for CibouletteCreateRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
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

        let CibouletteBody {
            data,
            meta,
            links,
            jsonapi,
            ..
        } = body.ok_or(CibouletteError::NoData)?;

        let data = match data {
            CibouletteBodyData::Object(x) => x,
            CibouletteBodyData::Null(_) => return Err(CibouletteError::NoData),
        };
        let data = match data {
            CibouletteResourceSelector::One(data) => {
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
            links,
            jsonapi,
            expected_response_type: CibouletteResponseRequiredType::Object(
                CibouletteResponseQuantity::Single,
            ),
        })
    }
}
