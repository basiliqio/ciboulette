use super::*;

pub type CibouletteResponseBodyData<'request, B> =
    CibouletteOptionalData<CibouletteResponseResourceSelector<'request, B>>;

/// ## `JSON:API` response body
#[derive(Debug, Getters, MutGetters, Clone, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResponseBody<'request, B> {
    /// The `JSON:API` server semver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    /// The object links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteBodyLink<'request>>,
    /// The object/relationships data
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    pub data: CibouletteResponseBodyData<'request, B>,
    /// An error, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<CibouletteErrorObj<'request>>,
    /// The included lists
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<CibouletteResponseResource<'request, B>>,
}

/// ## Resource included in a response
#[derive(Debug, Getters, MutGetters, Clone, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResponseResource<'request, B> {
    #[serde(flatten)]
    pub identifier: CibouletteResourceResponseIdentifier<'request>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<B>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub relationships: BTreeMap<ArcStr, CibouletteResponseRelationshipObject<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing)]
    pub type_: Arc<CibouletteResourceType>,
}

/// ## Relationships object included in a response
#[derive(Debug, Serialize, Getters, MutGetters, Default, Clone)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct CibouletteResponseRelationshipObject<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    pub data: CibouletteOptionalData<CibouletteResourceResponseIdentifierSelector<'request>>,
}

/// ## Selector for [CibouletteResponseResource](CibouletteResponseResource)
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CibouletteResponseResourceSelector<'request, B> {
    One(CibouletteResponseResource<'request, B>),
    Many(Vec<CibouletteResponseResource<'request, B>>),
}
