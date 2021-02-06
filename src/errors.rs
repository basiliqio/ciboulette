use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum CibouletteError {
    // #[error("The OpenApi document check has failed at `{0}`: {1}")]
    // OApiCheck(String, String),
    #[error("The json:api type `{0}` is unknown.")]
    UnknownType(String),
    /// When there is a failure while deserializing the JSON
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
