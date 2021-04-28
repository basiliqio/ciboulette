/// The quantity of data that should be returned
#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseQuantity {
    /// A single object
    Single,
    /// An array of objects, potentially empty
    Multiple,
}

/// The format the return object should have
#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseRequiredType {
    /// An object with its attributes
    Object(CibouletteResponseQuantity),
    /// An object with only its identifiers
    Id(CibouletteResponseQuantity),
    /// Nothing should be returned
    None,
}
