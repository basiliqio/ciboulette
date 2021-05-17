use super::*;

mod create;
mod read;

#[cfg(test)]
mod tests;

pub fn create_link<S1, S2, S3>(
    config: &CibouletteConfig,
    type_: S1,
    id: Option<S2>,
    is_related: bool,
    final_type: Option<S3>,
) -> String
where
    S1: std::fmt::Display,
    S2: std::fmt::Display,
    S3: std::fmt::Display,
{
    match (config.base_url(), id, final_type) {
        (None, Some(id), Some(final_type)) if is_related => {
            format!("/{}/{}/{}", type_, id, final_type)
        }
        (None, Some(id), Some(final_type)) => {
            format!("/{}/{}/relationships/{}", type_, id, final_type)
        }
        (None, None, Some(_)) => unreachable!(),
        (None, Some(id), None) => format!("/{}/{}", type_, id),
        (None, None, None) => format!("/{}", type_),
        (Some(base_url), Some(id), Some(final_type)) if is_related => {
            format!("{}/{}/{}/{}", base_url, type_, id, final_type)
        }
        (Some(base_url), Some(id), Some(final_type)) => {
            format!("{}/{}/{}/relationships/{}", base_url, type_, id, final_type)
        }
        (Some(_), None, Some(_)) => unreachable!(),
        (Some(base_url), Some(id), None) => format!("{}/{}/{}", base_url, type_, id),
        (Some(base_url), None, None) => format!("{}/{}", base_url, type_),
    }
}

pub(crate) fn build_link_for_response_root<'result, 'store, 'request>(
    config: &'store CibouletteConfig,
    inbound_request: &'request dyn CibouletteRequestCommons<'request>,
) -> Option<CibouletteLink<'result>> {
    if config.gen_root_links() {
        match inbound_request.intention() {
            CibouletteIntention::Create => Some(create::root_link(config, inbound_request)),
            CibouletteIntention::Read => Some(read::root_link(config, inbound_request)),
            CibouletteIntention::Update => Some(read::root_link(config, inbound_request)),
            CibouletteIntention::Delete => None,
        }
    } else {
        None
    }
}

pub(crate) fn build_link_for_response_object<'result, 'store, 'request>(
    config: &'store CibouletteConfig,
    identifier: &'request CibouletteResourceResponseIdentifier<'request>,
) -> Option<CibouletteLink<'result>> {
    if config.gen_resource_links() {
        Some(CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link::<
                _,
                _,
                &str,
            >(
                config,
                identifier.type_(),
                Some(identifier.id()),
                false,
                None,
            )))),
            related: None,
        })
    } else {
        None
    }
}

pub(crate) fn build_link_for_response_relationship<'result, 'store, 'request>(
    config: &'store CibouletteConfig,
    relates_to: &'request CibouletteResourceResponseIdentifier<'request>,
    relationship_name: &ArcStr,
) -> Option<CibouletteLink<'result>> {
    if config.gen_relationship_links() {
        Some(CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link(
                config,
                relates_to.type_(),
                Some(relates_to.id()),
                false,
                Some(relationship_name.clone()),
            )))),
            related: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link(
                config,
                relates_to.type_(),
                Some(relates_to.id()),
                true,
                Some(relationship_name.clone()),
            )))),
        })
    } else {
        None
    }
}
