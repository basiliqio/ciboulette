/// ## Represent the client intention (request method) when sending the request
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CibouletteIntention {
    /// Corresponds to a `POST` request
    Create,
    /// Corresponds to a `PATCH` request
    Update,
    /// Corresponds to a `GET` request
    Read,
    /// Corresponds to a `DELETE` request
    Delete,
}

impl std::fmt::Display for CibouletteIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CibouletteIntention::Create => write!(f, "Create"),
            CibouletteIntention::Update => write!(f, "Update"),
            CibouletteIntention::Read => write!(f, "Read"),
            CibouletteIntention::Delete => write!(f, "Delete"),
        }
    }
}
