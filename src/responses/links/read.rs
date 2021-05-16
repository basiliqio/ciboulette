use super::*;

pub fn root_link<'result, 'store, 'request>(
    config: &'store CibouletteConfig,
    inbound_request: &'request dyn CibouletteRequestCommons<'request>,
) -> CibouletteLink<'result> {
    match inbound_request.path() {
        CiboulettePath::Type(type_) => CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link::<
                &str,
                &str,
                &str,
            >(
                config,
                type_.name().as_str(),
                None,
                false,
                None,
            )))),
            related: None,
        },
        CiboulettePath::TypeId(type_, id) => CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link::<
                &str,
                _,
                &str,
            >(
                config,
                type_.name().as_str(),
                Some(id),
                false,
                None,
            )))),
            related: None,
        },
        CiboulettePath::TypeIdRelationship(type_, id, rel_details) => CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link(
                config,
                type_.name().as_str(),
                Some(id),
                false,
                Some(rel_details.related_type().name().as_str()),
            )))),
            related: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link(
                config,
                type_.name().as_str(),
                Some(id),
                true,
                Some(rel_details.related_type().name().as_str()),
            )))),
        },
        CiboulettePath::TypeIdRelated(type_, id, rel_details) => CibouletteLink {
            self_: Some(CibouletteLinkSelector::Simple(Cow::Owned(create_link(
                config,
                type_.name().as_str(),
                Some(id),
                true,
                Some(rel_details.related_type().name().as_str()),
            )))),
            related: None,
        },
    }
}
