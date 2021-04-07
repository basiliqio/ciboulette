use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteCreateRequest<'request, 'store> {
    pub path: CiboulettePath<'request, 'store>,
    pub query: CibouletteQueryParameters<'request, 'store>,
    pub data: CibouletteResource<
        'request,
        'store,
        MessyJsonObjectValue<'store>,
        CibouletteResourceIdentifierPermissive<'request>,
    >,
    pub meta: Option<Value>,
    pub links: Option<CibouletteBodyLink<'request>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>,
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request, 'store> CibouletteInboundRequestCommons<'request, 'store>
    for CibouletteCreateRequest<'request, 'store>
{
    fn path(&self) -> &CiboulettePath<'request, 'store> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request, 'store> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Create
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request, 'store> TryFrom<CibouletteInboundRequest<'request, 'store>>
    for CibouletteCreateRequest<'request, 'store>
{
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request, 'store>) -> Result<Self, Self::Error> {
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
