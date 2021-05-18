use super::*;

#[test]
fn iter_single() {
    let selector = CibouletteSelector::new_single("hello".to_string());

    let res: Vec<String> = selector.into_iter().collect();
    assert_eq!(res, vec!["hello".to_string()]);
}

#[test]
fn iter_multi() {
    let selector = CibouletteSelector::new_multi(vec!["hello".to_string(), "world".to_string()]);

    let res: Vec<String> = selector.into_iter().collect();
    assert_eq!(res, vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn iter_ref_single() {
    let selector = CibouletteSelector::new_single("hello".to_string());
    let strs = "hello".to_string();

    let res: Vec<&String> = selector.iter().collect();
    assert_eq!(res, vec![&strs]);
}

#[test]
fn iter_ref_multi() {
    let selector = CibouletteSelector::new_multi(vec!["hello".to_string(), "world".to_string()]);
    let strs = ("hello".to_string(), "world".to_string());

    let res: Vec<&String> = selector.iter().collect();
    assert_eq!(res, vec![&strs.0, &strs.1]);
}
