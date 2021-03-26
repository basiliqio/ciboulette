#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseQuantity {
    Single,
    Multiple,
}

#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseRequiredType {
    Object(CibouletteResponseQuantity),
    Id(CibouletteResponseQuantity),
    None,
}
