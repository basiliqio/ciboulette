use super::*;

#[test]
fn get_single() {
    let selector = CibouletteSelector::new_single("world".to_string());

    assert_eq!(selector.get(0).unwrap().as_str(), "world");
}

#[test]
fn get_single_out_of_bound() {
    let selector = CibouletteSelector::new_single("world".to_string());

    assert_eq!(
        matches!(selector.get(1), Err(CibouletteSelectorError::OutOfBound(x)) if x == 1),
        true
    );
}

#[test]
fn get_multi() {
    let selector = CibouletteSelector::new_multi(vec!["hello".to_string(), "world".to_string()]);

    assert_eq!(selector.get(0).unwrap().as_str(), "hello");
    assert_eq!(selector.get(1).unwrap().as_str(), "world");
    assert_eq!(selector.len(), 2);
    assert_eq!(selector.is_empty(), false);
}

#[test]
fn get_multi_out_of_bound() {
    let selector = CibouletteSelector::new_multi(vec!["hello".to_string(), "world".to_string()]);

    assert_eq!(
        matches!(selector.get(3), Err(CibouletteSelectorError::OutOfBound(x)) if x == 3),
        true
    );
}
