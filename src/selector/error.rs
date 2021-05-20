use thiserror::Error;

/// # An error throwable by [CibouletteSelector](crate::selector)
#[derive(Error, Debug)]
pub enum CibouletteSelectorError {
    #[error("The provided index `{0}` is out of bound")]
    OutOfBound(usize),
}
