use super::*;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteInboundRequestBuilder<'a> {
    req_url: &'a Url,
    intention: CibouletteIntention,
    body: &'a Option<&'a str>,
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteInboundRequest<'a> {
    pub path: CiboulettePath<'a>,
    pub query: CibouletteQueryParameters<'a>,
    pub body: Option<CibouletteBody<'a, MessyJsonObjectValue<'a>>>,
    pub intention: CibouletteIntention,
}

pub trait CibouletteInboundRequestCommons<'a> {
    fn path(&self) -> &CiboulettePath<'a>;
    fn query(&self) -> &CibouletteQueryParameters<'a>;
    fn intention(&self) -> CibouletteIntention;
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType;
}

impl<'a> CibouletteInboundRequestBuilder<'a> {
    pub fn new(
        intention: CibouletteIntention,
        req_url: &'a Url,
        body: &'a Option<&'a str>,
    ) -> Self {
        CibouletteInboundRequestBuilder {
            req_url,
            body,
            intention,
        }
    }

    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteInboundRequest<'a>, CibouletteError> {
        let path = CiboulettePathBuilder::parse(self.req_url)?.build(&bag)?;
        let body: Option<CibouletteBody<'a, MessyJsonObjectValue<'a>>> = match self.body {
            // Build body
            Some(body) => {
                let builder: CibouletteBodyBuilder<'_> = serde_json::from_str(body)?;
                Some(builder.build(bag, self.intention())?)
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

        Ok(CibouletteInboundRequest {
            path,
            body,
            query: query.unwrap_or_default(),
            intention: self.intention,
        })
    }
}
