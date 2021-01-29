use super::CibouletteError;

/// ## Check trait for JSON:API document
///
/// This trait allows struct of this crate to check themselves for logic errors
/// and/or specification violations.
pub trait OApiCheckTrait {
    fn check(&self) -> Result<(), CibouletteError>;
}
