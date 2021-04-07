use super::*;

#[derive(Debug, Clone)]
pub enum CibouletteUpdateRequestType<'request, 'store> {
    MainType(
        CibouletteResource<
            'request,
            'store,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifier<'request>,
        >,
    ),
    Relationship(CibouletteUpdateRelationship<'request, 'store>),
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRelationship<'request, 'store> {
    type_: Arc<CibouletteResourceType<'store>>,
    value: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
}

#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'request, 'store> {
    pub resource_type: Arc<CibouletteResourceType<'store>>,
    pub resource_id: CibouletteId<'request>,
    pub related_type: Option<Arc<CibouletteResourceType<'store>>>,
    pub path: CiboulettePath<'request, 'store>,
    pub query: CibouletteQueryParameters<'request, 'store>,
    pub data: CibouletteUpdateRequestType<'request, 'store>,
    pub meta: Option<Value>,
    pub links: Option<CibouletteBodyLink<'request>>,
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request, 'store> CibouletteInboundRequestCommons<'request, 'store>
    for CibouletteUpdateRequest<'request, 'store>
{
    fn path(&self) -> &CiboulettePath<'request, 'store> {
        &self.path
    }
    fn query(&self) -> &CibouletteQueryParameters<'request, 'store> {
        &self.query
    }
    fn intention(&self) -> CibouletteIntention {
        CibouletteIntention::Update
    }
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType {
        &self.expected_response_type
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request, 'store> TryFrom<CibouletteInboundRequest<'request, 'store>>
    for CibouletteUpdateRequest<'request, 'store>
where
    'request: 'store,
{
    type Error = CibouletteError;

    fn try_from(value: CibouletteInboundRequest<'request, 'store>) -> Result<Self, Self::Error> {
        let query: CibouletteQueryParameters<'request, 'store> = value.query;
        let body: Option<
            CibouletteBody<
                'request,
                'store,
                CibouletteResourceIdentifierPermissive<'request>,
                MessyJsonObjectValue<'store>,
            >,
        > = value.body;
        let intention: CibouletteIntention = value.intention;
        let path: CiboulettePath<'request, 'store> = value.path;

        let (resource_type, resource_id, related_type): (
            Arc<CibouletteResourceType<'store>>,
            &CibouletteId,
            Option<Arc<CibouletteResourceType<'store>>>,
        ) = match &path {
            CiboulettePath::TypeId(type_, id) => (type_.clone(), id, None),
            CiboulettePath::TypeIdRelationship(type_, id, rel_type) => {
                (type_.clone(), id, Some(rel_type.clone()))
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
            CibouletteBodyData::Object(selector) => match related_type.clone() {
                Some(related_type) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationship {
                        type_: related_type.clone(),
                        value: CibouletteOptionalData::Object(selector.try_into()?),
                    })
                }
                None => match selector {
                    CibouletteResourceSelector::One(value) => {
                        let type_: CibouletteResource<
                            'request,
                            'store,
                            MessyJsonObjectValue<'store>,
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
            CibouletteBodyData::Null(present) => match related_type.clone() {
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
