use super::*;

/// ## Update request type
///
/// Can either be an update of an object or of a relationship
#[derive(Debug, Clone)]
pub enum CibouletteUpdateRequestType<'request> {
    MainType(
        CibouletteResource<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifier<'request>,
        >,
    ),
    Relationship(CibouletteUpdateRelationshipBody<'request>),
}

/// ## Body of a relationships update request
#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRelationshipBody<'request> {
    type_: Arc<CibouletteResourceType>,
    value: CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
}

/// An `UPDATE` request
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteUpdateRequest<'request> {
    /// The base type beeing updated
    pub resource_type: Arc<CibouletteResourceType>,
    /// The resource id on which the update is based
    pub resource_id: CibouletteId<'request>,
    /// If updating a relationships, the related type
    pub related_type: Option<Arc<CibouletteResourceType>>,
    /// The path used to query
    pub path: CiboulettePath<'request>,
    /// The query parameters included
    pub query: CibouletteQueryParameters<'request>,
    /// The update requests data provided by the client
    pub data: CibouletteUpdateRequestType<'request>,
    /// The meta data included by the client
    pub meta: Option<Value>,
    /// The expected response type for that request
    pub expected_response_type: CibouletteResponseRequiredType,
}

impl<'request> CibouletteRequestCommons<'request> for CibouletteUpdateRequest<'request> {
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

    fn anchor_type(&self) -> &Arc<CibouletteResourceType> {
        self.path().base_type()
    }

    fn meta(&self) -> &Option<serde_json::Value> {
        &self.meta
    }
}

impl<'request> TryFrom<CibouletteRequest<'request>> for CibouletteUpdateRequest<'request> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteRequest<'request>) -> Result<Self, Self::Error> {
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

        let CibouletteBody { data, meta, .. } = body.unwrap_or_default();
        let data = match data {
            CibouletteBodyData::Object(selector) => match related_type {
                Some(related_details) => {
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationshipBody {
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
                    CibouletteUpdateRequestType::Relationship(CibouletteUpdateRelationshipBody {
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
            expected_response_type: CibouletteResponseRequiredType::Object(
                CibouletteResponseQuantity::Single,
            ),
        })
    }
}
