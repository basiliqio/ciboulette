use super::*;

#[derive(Debug, Clone)]
pub enum CibouletteUpdateRequestType<'a> {
    MainType(CibouletteResource<'a, CibouletteResourceIdentifier<'a>>),
    Relationship(
        &'a CibouletteResourceType<'a>,
        CibouletteResourceIdentifierSelector<'a>,
    ),
}

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'a> {
    pub resource_type: &'a CibouletteResourceType<'a>,
    pub resource_id: Cow<'a, str>,
    pub related_type: Option<&'a CibouletteResourceType<'a>>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteUpdateRequestType<'a>,
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

        let (resource_type, resource_id, related_type) = match path {
            CiboulettePath::TypeId(type_, id) => (type_, id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => (type_, id, Some(rel_type)),
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
            CibouletteBodyData::Object(CibouletteResourceSelector::One(data)) => match related_type
            {
                Some(related_type) => {
                    CibouletteUpdateRequestType::Relationship(related_type, data.try_into()?)
                }
                None => CibouletteUpdateRequestType::MainType(data.try_into()?),
            },
            CibouletteBodyData::Object(CibouletteResourceSelector::Many(_)) => {
                return Err(CibouletteError::NoCompound)
            }
            CibouletteBodyData::Null(_) => return Err(CibouletteError::NoData),
        };
        Ok(CibouletteUpdateRequest {
            resource_type,
            resource_id,
            related_type,
            query: query.unwrap_or_default(),
            data,
            meta,
            links,
            jsonapi,
        })
    }
}
