use super::*;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteRequestBuilder<'request> {
    /// The request URL
    req_url: &'request Url,
    /// The method used
    intention: CibouletteIntention,
    /// The body, if any
    body: &'request Option<&'request str>,
}

/// ## Abstract representation of a `JSON:API` request
#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteRequest<'request> {
    /// The path used for the query
    pub path: CiboulettePath<'request>,
    /// The query parameter included
    pub query: CibouletteQueryParameters<'request>,
    /// The request body if any
    pub body: Option<
        CibouletteBody<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
    >,
    /// The method used
    pub intention: CibouletteIntention,
}

/// ## `JSON:API` inbound requests
pub trait CibouletteRequestCommons<'request>: Send + Sync {
    /// Get a reference to request path
    fn path(&self) -> &CiboulettePath<'request>;
    /// Get a reference to request query parameters
    fn query(&self) -> &CibouletteQueryParameters<'request>;
    /// Get a reference to the request intention (method)
    fn intention(&self) -> CibouletteIntention;
    /// Get the expected response type for that request
    fn expected_response_type(&self) -> &CibouletteResponseRequiredType;
    /// The expected response type when building a response for that request
    fn expected_type(&self) -> &Arc<CibouletteResourceType>;
    /// The type on which relationships should be based on
    fn anchor_type(&self) -> &Arc<CibouletteResourceType>;
    /// Meta data included by the client, if any
    fn meta(&self) -> &Option<serde_json::Value>;
}

impl<'request> CibouletteRequestBuilder<'request> {
    /// Create a new inbound requests from parts
    pub fn new(
        intention: CibouletteIntention,
        req_url: &'request Url,
        body: &'request Option<&'request str>,
    ) -> Self {
        CibouletteRequestBuilder {
            req_url,
            intention,
            body,
        }
    }

    /// Build the inbound request, checking its validity and parsing the inner body
    ///
    /// Once built, this request can be transformed into the definitive request depending
    /// on its intention.
    pub fn build(
        self,
        bag: &CibouletteStore,
    ) -> Result<CibouletteRequest<'request>, CibouletteError> {
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

        Ok(CibouletteRequest {
            path,
            body,
            query: query.unwrap_or_default(),
            intention: self.intention,
        })
    }
}
