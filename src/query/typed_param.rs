use super::*;

pub fn parse_typed_query_params<'a>(s: &str) -> Option<Vec<Cow<'a, str>>> {
    if let Some(type_) = parse_typed_query_param(s) {
        let res: Vec<Cow<'a, str>> = type_
            .as_ref()
            .split('.')
            .map(str::to_string)
            .map(Cow::Owned)
            .collect();
        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn parse_typed_query_param(s: &str) -> Option<Cow<'_, str>> {
    if s.len() <= 2 || !(s.starts_with('[') && s.ends_with(']')) {
        return None;
    }
    Some(Cow::Borrowed(&s[1..s.len() - 1]))
}

#[cfg(test)]
mod tests_params {
    use super::*;

    #[test]
    fn ok_simple() {
        let input = "[toto]";
        let res = parse_typed_query_params(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res.len(), 1, "should have a single type");
        assert_eq!(res.get(0).unwrap(), &"toto", "type mismatch");
    }

    #[test]
    fn no_match() {
        let input = "AAAAAAAAAA";
        let res = parse_typed_query_params(&input);
        assert_eq!(
            res.is_none(),
            true,
            "shouldn't have parsed the sparse field param"
        );
    }

    #[test]
    fn no_type() {
        let input = "[]";
        let res = parse_typed_query_params(&input);
        assert_eq!(
            res.is_none(),
            true,
            "shouldn't have parsed the sparse field param"
        );
    }

    #[test]
    fn nested_type_1() {
        let input = "[toto.tutu]";
        let res = parse_typed_query_params(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res.len(), 2, "should have a two types");
        assert_eq!(res.get(0).unwrap(), &"toto", "type mismatch");
        assert_eq!(res.get(1).unwrap(), &"tutu", "type mismatch");
    }

    #[test]
    fn nested_type_n() {
        let input = "[toto.tutu.toto.tata]";
        let res = parse_typed_query_params(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res.len(), 4, "should have a two types");
        assert_eq!(res.get(0).unwrap(), &"toto", "type mismatch");
        assert_eq!(res.get(1).unwrap(), &"tutu", "type mismatch");
        assert_eq!(res.get(2).unwrap(), &"toto", "type mismatch");
        assert_eq!(res.get(3).unwrap(), &"tata", "type mismatch");
    }
}

#[cfg(test)]
mod tests_param {
    use super::*;

    #[test]
    fn ok_simple() {
        let input = "[toto]";
        let res = parse_typed_query_param(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res, "toto", "type mismatch");
    }

    #[test]
    fn no_match() {
        let input = "AAAAAAAAAA";
        let res = parse_typed_query_param(&input);
        assert_eq!(
            res.is_none(),
            true,
            "shouldn't have parsed the sparse field param"
        );
    }

    #[test]
    fn no_type() {
        let input = "[]";
        let res = parse_typed_query_param(&input);
        assert_eq!(
            res.is_none(),
            true,
            "shouldn't have parsed the sparse field param"
        );
    }

    #[test]
    fn nested_type_1() {
        let input = "[toto.tutu]";
        let res = parse_typed_query_param(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res, "toto.tutu", "type mismatch");
    }

    #[test]
    fn nested_type_n() {
        let input = "[toto.tutu.toto.tata]";
        let res = parse_typed_query_param(&input);
        assert_eq!(
            res.is_some(),
            true,
            "should have parsed the sparse field param"
        );
        let res = res.unwrap();
        assert_eq!(res, "toto.tutu.toto.tata", "type mismatch");
    }
}
