use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TYPED_PARAM_REGEX: Regex = Regex::new(r"^\[(?P<type>.+)*\]").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let input = "[toto]";
        let mut res = TYPED_PARAM_REGEX.captures_iter(&input);
        let whole_match = res.next().expect("the main type");
        assert_eq!(
            whole_match.get(1).unwrap().as_str(),
            "toto",
            "Main type mismatch"
        );
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }

    #[test]
    fn no_match() {
        let input = "AAAAAAAAAA";
        let mut res = TYPED_PARAM_REGEX.captures_iter(&input);
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }

    #[test]
    fn no_type() {
        let input = "[]";
        let mut res = TYPED_PARAM_REGEX.captures_iter(&input);
        let whole_match = res.next().expect("the main type");
        assert_eq!(
            matches!(whole_match.get(1), None),
            true,
            "Shouldn't have a type present"
        );
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }

    #[test]
    fn nested_type_1() {
        let input = "[toto.tutu]";
        let mut res = TYPED_PARAM_REGEX.captures_iter(&input);
        let whole_match = res.next().expect("the main type");
        assert_eq!(
            whole_match.get(1).unwrap().as_str(),
            "toto.tutu",
            "Main type mismatch"
        );
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }

    #[test]
    fn nested_type_n() {
        let input = "[toto.tutu.toto.tata]";
        let mut res = TYPED_PARAM_REGEX.captures_iter(&input);
        let whole_match = res.next().expect("the main type");
        assert_eq!(
            whole_match.get(1).unwrap().as_str(),
            "toto.tutu.toto.tata",
            "Main type mismatch"
        );
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }
}
