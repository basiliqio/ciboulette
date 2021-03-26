use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteCreateRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteResource<
        'a,
        MessyJsonObjectValue<'a>,
        CibouletteResourceIdentifierPermissive<'a>,
    >,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'a>>,
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'a> CibouletteInboundRequestCommons<'a> for CibouletteCreateRequest<'a> {
    fn path(&self) -> &CiboulettePath<'a> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'a> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Create
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &serde_json::Value {
        &self.meta
    }
}

impl<'a> TryFrom<CibouletteInboundRequest<'a>> for CibouletteCreateRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'a>) -> Result<Self, Self::Error> {
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
