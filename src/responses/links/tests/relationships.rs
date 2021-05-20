use super::*;

#[test]
fn simple() {
    let store = gen_bag();

    let id = Uuid::new_v4();
    let rel_name = arcstr::literal!("myrel");
    let link = crate::responses::links::build_link_for_response_relationship(
        store.config(),
        &CibouletteResourceResponseIdentifier {
            type_: arcstr::literal!("mytype"),
            id: CibouletteIdSelector::new(CibouletteSelector::Single(CibouletteId::Uuid(id))),
        },
        &rel_name,
    )
    .unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("/mytype/{}/relationships/myrel", id)),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("/mytype/{}/myrel", id)),
        true
    );
}

#[test]
fn simple_with_base_url() {
    let mut store = gen_bag();

    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());
    let id = Uuid::new_v4();
    let rel_name = arcstr::literal!("myrel");
    let link = crate::responses::links::build_link_for_response_relationship(
        store.config(),
        &CibouletteResourceResponseIdentifier {
            type_: arcstr::literal!("mytype"),
            id: CibouletteIdSelector::new(CibouletteSelector::Single(CibouletteId::Uuid(id))),
        },
        &rel_name,
    )
    .unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("http://localhost:80/mytype/{}/relationships/myrel", id)),
        true
    );
    assert_eq!(
        matches!(link.related(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("http://localhost:80/mytype/{}/myrel", id)),
        true
    );
}
