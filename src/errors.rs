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

/// # An error throwable by [Ciboulette](crate)
#[derive(Error, Debug)]
pub enum CibouletteError {
    #[error("The json:api type `{0}` is unknown.")]
    UnknownType(String),
    #[error("No relationship `{1}` for type `{0}`.")]
    UnknownRelationship(String, String),
    #[error("No field {1} for type {0}.")]
    UnknownField(String, String),
    #[error("The current document doesn't allow sorting parameters")]
    IncompatibleSorting,
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
    #[error("Type `{0}` is not in the graph")]
    RelNotInGraph(String),
    #[error("The key `{0}` must be present {1} `{2}`")]
    KeyClash(String, CibouletteClashDirection, String),
    #[error("The member name {0} doesn't respect the json:api specification")]
    InvalidMemberName(String),
    #[error("The `attributes` should be an object")]
    AttributesIsNotAnObject,
    /// When there is a failure while deserializing the JSON
    #[error("An unkown error occured : {0}")]
    UnknownError(String),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrlEncoded(#[from] serde_urlencoded::de::Error),
}
