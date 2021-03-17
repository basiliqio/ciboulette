use super::*;

#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct CibouletteReadRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'a> TryFrom<CibouletteRequest<'a>> for CibouletteReadRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteRequest {
            query,
            body,
            intention,
            path,
        } = value;
        let expected_response_type: CibouletteResponseRequiredType = match path {
            CiboulettePath::Type(_)
            | CiboulettePath::TypeId(_, _)
            | CiboulettePath::TypeIdRelated(_, _, _) => CibouletteResponseRequiredType::Object,
            CiboulettePath::TypeIdRelationship(_, _, _) => CibouletteResponseRequiredType::Id,
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
            CibouletteBodyData::Object(x) => x,
            CibouletteBodyData::Null(_) => CibouletteResourceSelector::<
                CibouletteResourceIdentifierPermissive<'_>,
            >::Many(Vec::new()),
        }
        .try_into()?;
        Ok(CibouletteReadRequest {
            path,
            query: query.unwrap_or_default(),
            data,
            meta,
            links,
            jsonapi,
            expected_response_type,
        })
    }
}
