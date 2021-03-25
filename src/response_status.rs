#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseStatus {
    Ok,
    OkAsync,
    Unsupported,
    NotFound,
    Conflict,
}
