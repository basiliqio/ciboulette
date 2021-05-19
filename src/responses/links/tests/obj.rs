use super::*;

#[test]
fn simple_obj_link() {
    let store = gen_bag();

    let id = Uuid::new_v4();
    let link = crate::responses::links::build_link_for_response_object(
        store.config(),
        &CibouletteResourceResponseIdentifier {
            type_: arcstr::literal!("mytype"),
            id: CibouletteIdSelector::new(CibouletteSelector::Single(CibouletteId::Uuid(id))),
        },
    )
    .unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("/mytype/{}", id)),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}

#[test]
fn simple_obj_link_with_base_url() {
    let mut store = gen_bag();
    *store.config_mut().base_url_mut() = Some("http://localhost:80".to_string());

    let id = Uuid::new_v4();
    let link = crate::responses::links::build_link_for_response_object(
        store.config(),
        &CibouletteResourceResponseIdentifier {
            type_: arcstr::literal!("mytype"),
            id: CibouletteIdSelector::new(CibouletteSelector::Single(CibouletteId::Uuid(id))),
        },
    )
    .unwrap();

    assert_eq!(
        matches!(link.self_(), Some(CibouletteLinkSelector::Simple(x)) if x.as_ref() == format!("http://localhost:80/mytype/{}", id)),
        true
    );
    assert_eq!(matches!(link.related(), None), true);
}
