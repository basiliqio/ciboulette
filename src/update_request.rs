use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'a> {
    pub resource_type: &'a CibouletteResourceType<'a>,
    pub resource_id: Cow<'a, str>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: Option<CibouletteResource<'a, CibouletteResourceIdentifier<'a>>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

impl<'a> TryFrom<CibouletteRequest<'a>> for CibouletteUpdateRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteRequest {
            query,
            body,
            intention,
            path,
        } = value;

        let (resource_type, resource_id) = match path {
            CiboulettePath::TypeId(type_, id) => (type_, id),
            _ => {
                return Err(CibouletteError::WrongPathType(
                    CiboulettePathType::from(&path),
                    vec![CiboulettePathType::TypeId],
                ))
            }
        };

        if !matches!(intention, CibouletteIntention::Update) {
            return Err(CibouletteError::WrongIntention(
                intention,
                CibouletteIntention::Update,
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
            Some(CibouletteResourceSelector::One(data)) => Some(data.try_into()?),
            None => None,
            _ => return Err(CibouletteError::NoCompound),
        };

        Ok(CibouletteUpdateRequest {
            resource_type,
            resource_id,
            query: query.unwrap_or_default(),
            data,
            meta,
            links,
            jsonapi,
        })
    }
}
