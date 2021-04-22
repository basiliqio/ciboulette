use super::*;

#[derive(Debug, Clone)]
pub enum CibouletteUpdateRequestType<'request> {
    MainType(
        CibouletteResource<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifier<'request>,
        >,
    ),
    Relationship(CibouletteUpdateRelationship<'request>),
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRelationship<'request> {
    type_: Arc<CibouletteResourceType>,
    value: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
}

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'request> {
    pub resource_type: Arc<CibouletteResourceType>,
    pub resource_id: CibouletteId<'request>,
    pub related_type: Option<Arc<CibouletteResourceType>>,
    pub path: CiboulettePath<'request>,
    pub query: CibouletteQueryParameters<'request>,
    pub data: CibouletteUpdateRequestType<'request>,
    pub meta: Option<Value>,
    pub links: Option<CibouletteBodyLink<'request>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request> CibouletteInboundRequestCommons<'request> for CibouletteUpdateRequest<'request> {
    fn path(&self) -> &CiboulettePath<'request> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Update
    }

    fn expected_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().main_type()
    }

    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteInboundRequest<'request>> for CibouletteUpdateRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request>) -> Result<Self, Self::Error> {
        let query: CibouletteQueryParameters<'request> = value.query;
        let body: Option<
            CibouletteBody<
                'request,
                CibouletteResourceIdentifierPermissive<'request>,
                MessyJsonObjectValue<'request>,
            >,
        > = value.body;
        let intention: CibouletteIntention = value.intention;
        let path: CiboulettePath<'request> = value.path;

        let (resource_type, resource_id, related_type): (
            Arc<CibouletteResourceType>,
            &CibouletteId,
            Option<&CibouletteResourceRelationshipDetails>,
        ) = match &path {
            CiboulettePath::TypeId(type_, id) => (type_.clone(), id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => {
                (type_.clone(), id, Some(rel_type))
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
                Some(related_details) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationship {
                        type_: related_details.related_type().clone(),
                        value: CibouletteOptionalData::Object(selector.try_into()?),
                    })
                }
                None => match selector {
                    CibouletteResourceSelector::One(value) => {
                        let type_: CibouletteResource<
                            'request,
                            MessyJsonObjectValue<'request>,
                            CibouletteResourceIdentifier<'request>,
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
                Some(related_details) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationship {
                        type_: related_details.related_type().clone(),
                        value: CibouletteOptionalData::Null(present),
                    })
                }
                None => return Err(CibouletteError::NoData),
            },
        };
        Ok(CibouletteUpdateRequest {
            resource_type,
            resource_id: resource_id.clone(),
            related_type: related_type.map(|x| x.related_type()).cloned(),
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
