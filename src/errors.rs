use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum CibouletteError {
    // #[error("The OpenApi document check has failed at `{0}`: {1}")]
// OApiCheck(String, String),
}
