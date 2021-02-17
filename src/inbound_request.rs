use super::*;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Copy, Clone)]
pub enum CibouletteIntention {
    Create,
    Update,
    Read,
    Delete,
}

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteRequestBuilder<'a> {
    query: Option<&'a str>,
    body: Option<&'a str>,
    intention: CibouletteIntention,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteRequest<'a> {
    pub query: Option<CibouletteQueryParameters<'a>>,
    pub body: Option<CibouletteBody<'a>>,
    pub intention: CibouletteIntention,
}

impl<'a> CibouletteRequestBuilder<'a> {
    pub fn new(
        query: Option<&'a str>,
        body: Option<&'a str>,
        intention: CibouletteIntention,
    ) -> Self {
        CibouletteRequestBuilder {
            query,
            body,
            intention,
        }
    }

    pub fn build(self, bag: &'a CibouletteStore) -> Result<CibouletteRequest<'a>, CibouletteError> {
        let body: Option<CibouletteBody<'a>> = match self.body {
            // Build body
            Some(body) => {
                let builder: CibouletteBodyBuilder<'_> = serde_json::from_str(body)?;
                Some(builder.build(bag)?)
            }
            None => None,
        };

        let query: Option<CibouletteQueryParameters<'a>> = match self.query {
            // Build query parameters
            Some(query) => {
                let builder: CibouletteQueryParametersBuilder<'_> =
                    serde_urlencoded::from_str(query)?;
                Some(builder.build(bag, body.as_ref().and_then(|x| x.get_main_type(bag)))?)
            }
            None => None,
        };

        Ok(CibouletteRequest {
            body,
            query,
            intention: self.intention,
        })
    }
}
