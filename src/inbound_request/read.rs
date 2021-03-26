use super::*;

#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct CibouletteReadRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub data:
        CibouletteResourceSelector<'a, MessyJsonObjectValue<'a>, CibouletteResourceIdentifier<'a>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'a> CibouletteInboundRequestCommons<'a> for CibouletteReadRequest<'a> {
    fn path(&self) -> &CiboulettePath<'a> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'a> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Read
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &serde_json::Value {
        &self.meta
    }
}

impl<'a> TryFrom<CibouletteInboundRequest<'a>> for CibouletteReadRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
            query,
            body,
            intention,
            path,
        } = value;
        let expected_response_type: CibouletteResponseRequiredType = match path {
            CiboulettePath::Type(_)
            | CiboulettePath::TypeId(_, _)
            | CiboulettePath::TypeIdRelated(_, _, _) => {
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
                MessyJsonObjectValue<'a>,
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
