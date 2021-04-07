use super::*;

#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct CibouletteReadRequest<'request, 'store> {
    pub path: CiboulettePath<'request, 'store>,
    pub query: CibouletteQueryParameters<'request, 'store>,
    pub data: CibouletteResourceSelector<
        'request,
        'store,
        MessyJsonObjectValue<'store>,
        CibouletteResourceIdentifier<'request>,
    >,
    pub meta: Option<Value>,
    pub links: Option<CibouletteBodyLink<'request>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request, 'store> CibouletteInboundRequestCommons<'request, 'store>
    for CibouletteReadRequest<'request, 'store>
{
    fn path(&self) -> &CiboulettePath<'request, 'store> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request, 'store> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Read
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request, 'store> TryFrom<CibouletteInboundRequest<'request, 'store>>
    for CibouletteReadRequest<'request, 'store>
{
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request, 'store>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
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

        let CibouletteBody {
            data,
            meta,
            links,
            jsonapi,
            ..
        } = body.unwrap_or_default();

        let data = match data {
            CibouletteBodyData::Object(obj) => obj,
            CibouletteBodyData::Null(_) => CibouletteResourceSelector::<
                MessyJsonObjectValue<'store>,
                CibouletteResourceIdentifierPermissive<'_>,
            >::Many(Vec::new()),
        }
        .try_into()?;
        Ok(CibouletteReadRequest {
            path,
            query,
            data,
            meta,
            links,
            jsonapi,
            expected_response_type,
        })
    }
}
