use super::*;

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteCreateRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

impl<'a> TryFrom<CibouletteRequest<'a>> for CibouletteCreateRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'a>) -> Result<Self, Self::Error> {
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
            CibouletteResourceSelector::One(data) => data,
            _ => return Err(CibouletteError::NoCompound),
        };

        Ok(CibouletteCreateRequest {
            path,
            query: query.unwrap_or_default(),
            data,
            meta,
            links,
            jsonapi,
        })
    }
}
