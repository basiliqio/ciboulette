use super::*;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteInboundRequestBuilder<'request> {
    req_url: &'request Url,
    intention: CibouletteIntention,
    body: &'request Option<&'request str>,
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteInboundRequest<'request> {
    pub path: CiboulettePath<'request>,
    pub query: CibouletteQueryParameters<'request>,
    pub body: Option<
        CibouletteBody<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
    >,
    pub intention: CibouletteIntention,
}

pub trait CibouletteInboundRequestCommons<'request>: Send + Sync {
    fn path(&self) -> &CiboulettePath<'request>;
    fn query(&self) -> &CibouletteQueryParameters<'request>;
    fn intention(&self) -> CibouletteIntention;
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType;

    fn meta(&self) -> &Option<serde_json::Value>;
}

impl<'request> CibouletteInboundRequestBuilder<'request> {
    pub fn new(
        intention: CibouletteIntention,
        req_url: &'request Url,
        body: &'request Option<&'request str>,
    ) -> Self {
        CibouletteInboundRequestBuilder {
            req_url,
            body,
            intention,
        }
    }

    pub fn build(
        self,
        bag: &CibouletteStore,
    ) -> Result<CibouletteInboundRequest<'request>, CibouletteError> {
        let path: CiboulettePath<'request> =
            CiboulettePathBuilder::parse(self.req_url)?.build(&bag)?;
        let body: Option<
            CibouletteBody<
                'request,
                CibouletteResourceIdentifierPermissive<'request>,
                MessyJsonObjectValue<'request>,
            >,
        > = match self.body {
            // Build body
            Some(body) => {
                let builder: CibouletteBodyBuilder<'request> = serde_json::from_str(body)?;
                Some(builder.build(bag, self.intention())?)
            }
            None => None,
        };

        let query: Option<CibouletteQueryParameters<'request>> = match self.req_url.query() {
            // Build query parameters
            Some(query) => {
                let builder: CibouletteQueryParametersBuilder<'request> =
                    serde_urlencoded::from_str(query)?;
                Some(builder.build(bag, path.main_type().clone())?)
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
