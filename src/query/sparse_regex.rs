use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SPARSE_REGEX: Regex = Regex::new(r"^fields\[(?P<type>.+)*\]").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let input = "fields[toto]";
        let mut res = SPARSE_REGEX.captures_iter(&input);
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
        let mut res = SPARSE_REGEX.captures_iter(&input);
        assert_eq!(
            matches!(res.next(), None),
            true,
            "There should be no more matches"
        );
    }

    #[test]
    fn no_type() {
        let input = "fields[]";
        let mut res = SPARSE_REGEX.captures_iter(&input);
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
        let input = "fields[toto.tutu]";
        let mut res = SPARSE_REGEX.captures_iter(&input);
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
        let input = "fields[toto.tutu.toto.tata]";
        let mut res = SPARSE_REGEX.captures_iter(&input);
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
