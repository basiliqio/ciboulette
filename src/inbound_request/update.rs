use super::*;

#[derive(Debug, Clone)]
pub enum CibouletteUpdateRequestType<'a> {
    MainType(CibouletteResource<'a, MessyJsonObjectValue<'a>, CibouletteResourceIdentifier<'a>>),
    Relationship(CibouletteUpdateRelationship<'a>),
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRelationship<'a> {
    type_: &'a CibouletteResourceType<'a>,
    value: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'a>>,
}

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'a> {
    pub resource_type: &'a CibouletteResourceType<'a>,
    pub resource_id: CibouletteId<'a>,
    pub related_type: Option<&'a CibouletteResourceType<'a>>,
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub data: CibouletteUpdateRequestType<'a>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'a>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'a> CibouletteInboundRequestCommons<'a> for CibouletteUpdateRequest<'a> {
    fn path(&self) -> &CiboulettePath<'a> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'a> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Update
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &serde_json::Value {
        &self.meta
    }
}

impl<'a> TryFrom<CibouletteInboundRequest<'a>> for CibouletteUpdateRequest<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'a>) -> Result<Self, Self::Error> {
        let CibouletteInboundRequest {
            query,
            body,
            intention,
            path,
        } = value;

        let (resource_type, resource_id, related_type) = match &path {
            CiboulettePath::TypeId(type_, id) => (*type_, id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => {
                (*type_, id, Some(*rel_type))
            }
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
            CibouletteBodyData::Object(selector) => match related_type {
                Some(related_type) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationship {
                        type_: related_type,
                        value: CibouletteOptionalData::Object(selector.try_into()?),
                    })
                }
                None => match selector {
                    CibouletteResourceSelector::One(value) => {
                        let type_: CibouletteResource<
                            'a,
                            MessyJsonObjectValue<'a>,
                            CibouletteResourceIdentifier<'a>,
                        > = value.try_into()?;
                        if type_.identifier().type_() != path.main_type().name() {
                            return Err(CibouletteError::MainTypeClash);
                        }
                        CibouletteUpdateRequestType::MainType(type_)
                    }
                    CibouletteResourceSelector::Many(_) => return Err(CibouletteError::NoCompound),
                },
            },
            CibouletteBodyData::Null(present) => match related_type {
                Some(related_type) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationship {
                        type_: related_type,
                        value: CibouletteOptionalData::Null(present),
                    })
                }
                None => return Err(CibouletteError::NoData),
            },
        };
        Ok(CibouletteUpdateRequest {
            resource_type,
            resource_id: resource_id.clone(),
            related_type,
            query,
            data,
            meta,
            path,
            links,
            jsonapi,
            expected_response_type: CibouletteResponseRequiredType::Object(
                CibouletteResponseQuantity::Single,
            ),
        })
    }
}
