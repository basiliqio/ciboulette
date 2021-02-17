use super::*;

#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct CibouletteReadRequest<'a> {
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

impl<'a> TryFrom<CibouletteRequest<'a>> for CibouletteReadRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteRequest { query, body, .. } = value;
        let CibouletteBody {
            data,
            meta,
            links,
            jsonapi,
            ..
        } = body.unwrap_or_default();

        Ok(CibouletteReadRequest {
            query: query.unwrap_or_default(),
            data: data
                .unwrap_or_else(|| {
                    CibouletteResourceSelector::<CibouletteResourceIdentifierPermissive<'_>>::Many(
                        Vec::new(),
                    )
                })
                .try_into()?,
            meta,
            links,
            jsonapi,
        })
    }
}
