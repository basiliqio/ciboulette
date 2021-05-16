use super::*;

pub fn root_link<'result, 'store, 'request>(
    config: &'store CibouletteConfig,
    inbound_request: &'request dyn CibouletteRequestCommons<'request>,
) -> CibouletteLink<'result> {
    CibouletteLink {
        self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link::<
            _,
            &str,
            &str,
        >(
            config,
            inbound_request.expected_type().name(),
            None,
            false,
            None,
        )))),
        related: None,
    }
}
