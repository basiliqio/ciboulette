use super::*;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CibouletteIntention {
    Create,
    Update,
    Read,
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

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteRequestBuilder<'a> {
    req_url: &'a Url,
    intention: CibouletteIntention,
    body: &'a Option<&'a str>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: Option<CibouletteQueryParameters<'a>>,
    pub body: Option<CibouletteBody<'a>>,
    pub intention: CibouletteIntention,
}

impl<'a> CibouletteRequestBuilder<'a> {
    pub fn new(
        intention: CibouletteIntention,
        req_url: &'a Url,
        body: &'a Option<&'a str>,
    ) -> Self {
        CibouletteRequestBuilder {
            req_url,
            body,
            intention,
        }
    }

    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteRequest<'a>, CibouletteError> {
        let path = CiboulettePathBuilder::parse(self.req_url)?.build(&bag)?;
        let body: Option<CibouletteBody<'a>> = match self.body {
            // Build body
            Some(body) => {
                let builder: CibouletteBodyBuilder<'_> = serde_json::from_str(body)?;
                Some(builder.build(bag, self.intention(), path.main_type())?)
            }
            None => None,
        };

        let query: Option<CibouletteQueryParameters<'a>> = match self.req_url.query() {
            // Build query parameters
            Some(query) => {
                let builder: CibouletteQueryParametersBuilder<'a> =
                    serde_urlencoded::from_str(query)?;
                Some(builder.build(bag, Some(path.main_type()))?)
            }
            None => None,
        };

        Ok(CibouletteRequest {
            path,
            body,
            query,
            intention: self.intention,
        })
    }
}
