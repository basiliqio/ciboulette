#[derive(Debug, Clone, Copy)]
pub enum CibouletteResponseStatus {
    Ok,
    OkEmpty,
    OkAsync,
    Unsupported,
    NotFound,
    Conflict,
}
