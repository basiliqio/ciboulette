use thiserror::Error;

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

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum CibouletteError {
    #[error("The json:api type `{0}` is unknown.")]
    UnknownType(String),
    #[error("The resource object ({0}, {1}) should be unique by `type` and `id`")]
    UniqObj(String, String),
    #[error("The resource relationship ({0}, {1}) should be unique by `type` and `id`")]
    UniqRelationship(String, String),
    #[error("The linked object ({0}, {1}) is missing")]
    MissingLink(String, String),
    #[error("The linked object ({0}, {1}) is not completely linked")]
    NoCompleteLinkage(String, String),
    #[error("The key `{0}` must be present {1} `{2}`")]
    KeyClash(String, CibouletteClashDirection, String),
    /// When there is a failure while deserializing the JSON
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
