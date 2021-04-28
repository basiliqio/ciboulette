use super::*;
use thiserror::Error;

/// When describing KeyClash, we need to know if the field was supposed be with
/// or without another field
#[derive(Debug, PartialEq)]
pub enum CibouletteClashDirection {
    With,
    Without,
}

impl std::fmt::Display for CibouletteClashDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CibouletteClashDirection::With => "with",
                CibouletteClashDirection::Without => "without",
            }
        )
    }
}

/// ## `JSON:API` Path type
#[derive(Debug, PartialEq)]
pub enum CiboulettePathType {
    Type,
    TypeId,
    TypeIdRelated,
    TypeIdRelationship,
}

impl std::fmt::Display for CiboulettePathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CiboulettePathType::Type => write!(f, "Type"),
            CiboulettePathType::TypeId => write!(f, "TypeId"),
            CiboulettePathType::TypeIdRelated => write!(f, "TypeIdRelated"),
            CiboulettePathType::TypeIdRelationship => write!(f, "TypeIdRelationship"),
        }
    }
}

impl From<&CiboulettePath<'_>> for CiboulettePathType {
    fn from(t: &CiboulettePath<'_>) -> Self {
        match t {
            CiboulettePath::Type(_) => CiboulettePathType::Type,
            CiboulettePath::TypeId(_, _) => CiboulettePathType::TypeId,
            CiboulettePath::TypeIdRelated(_, _, _) => CiboulettePathType::TypeIdRelated,
            CiboulettePath::TypeIdRelationship(_, _, _) => CiboulettePathType::TypeIdRelationship,
        }
    }
}

/// # An error throwable by [Ciboulette](crate)
#[derive(Error, Debug)]
pub enum CibouletteError {
    #[error("The main type in the path and in the body are not the same")]
    MainTypeClash,
    #[error("The json:api type `{0}` is unknown.")]
    UnknownType(String),
    #[error("No relationship `{1}` for type `{0}`.")]
    UnknownRelationship(String, String),
    #[error("No field {1} for type {0}.")]
    UnknownField(String, String),
    #[error("The resource object ({0}, {1}) should be unique by `type` and `id`")]
    UniqObj(String, String),
    #[error("The type `{0}` is already defined")]
    UniqType(String),
    #[error("The resource relationship ({0}, {1}) should be unique by `type` and `id`")]
    UniqRelationshipObject(String, String),
    #[error("The relationship between {0} and {1} already exists")]
    UniqRelationship(String, String),
    #[error("The linked object ({0}, {1}) is missing")]
    MissingLink(String, String),
    #[error("The linked object ({0}, {1}) is not completely linked")]
    NoCompleteLinkage(String, String),
    #[error("Type `{0}` is not in the graph")]
    TypeNotInGraph(String),
    #[error("Relation `{0}`-`{1}` is not in the graph")]
    RelNotInGraph(String, String),
    #[error("The key `{0}` must be present {1} `{2}`")]
    KeyClash(String, CibouletteClashDirection, String),
    #[error("The member name {0} doesn't respect the json:api specification")]
    InvalidMemberName(String),
    #[error("The `attributes` should be an object")]
    AttributesIsNotAnObject,
    #[error("The `data` object is missing")]
    NoData,
    #[error("Missing `id` field for a resource identifier")]
    MissingId,
    #[error("Bad `id` type: found `{0}`, expected `{1}`")]
    BadIdType(CibouletteIdType, CibouletteIdType),
    #[error("Unkown id type `{0}`")]
    UnknownIdType(String),
    #[error("Compound documents are forbidden for that kind of request")]
    NoCompound,
    #[error("No alias translation for `{1}` in type `{0}`")]
    MissingAliasTranslation(String, String),
    #[error("Couldn't parse the path, no type were specified")]
    MissingTypeInPath,
    #[error("Couldn't parse the path")]
    BadPath,
    #[error("Wrong request intention, got {0}, expected {1}")]
    WrongIntention(CibouletteIntention, CibouletteIntention),
    #[error("Wrong path type, got {0}, expected {1:?}")]
    WrongPathType(CiboulettePathType, Vec<CiboulettePathType>),
    #[error("Too many main data to build the response document for type `{0}`")]
    ResponseTooManyMainData(String),
    /// When there is a failure while deserializing the JSON
    #[error("An unkown error occured : {0}")]
    UnknownError(String),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    SerdeUrlEncoded(#[from] serde_urlencoded::de::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}
