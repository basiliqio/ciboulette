#[inline]
fn check_member_name_borders(c: &char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        '\u{0020}'
        | '_'
        | '-'
        | '\u{002B}'
        | '\u{002C}'
        | '\u{002E}'
        | '\u{005B}'
        | '\u{005D}'
        | '\u{002F}'
        | '\u{005C}'
        | '\u{005E}'
        | '\u{0060}'
        | '\u{0000}'..='\u{001F}'
        | '\u{0021}'..='\u{002A}'
        | '\u{003A}'..='\u{0040}'
        | '\u{007B}'..='\u{007F}' => false,
        _ => true,
    }
}

#[inline]
fn check_member_name_corpus(c: &char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '\u{0020}' | '_' | '-' => true,
        '\u{002B}'
        | '\u{002C}'
        | '\u{002E}'
        | '\u{005B}'
        | '\u{005D}'
        | '\u{002F}'
        | '\u{005C}'
        | '\u{005E}'
        | '\u{0060}'
        | '\u{0000}'..='\u{001F}'
        | '\u{0021}'..='\u{002A}'
        | '\u{003A}'..='\u{0040}'
        | '\u{007B}'..='\u{007F}' => false,
        _ => true,
    }
}

#[inline]
pub fn check_member_name(s: &str) -> bool {
    match s.len() {
        0 => false,
        1..=2 => {
            for i in s.chars() {
                if !check_member_name_borders(&i) {
                    return false;
                }
            }
            true
        }
        _ => {
            let mut chars = s.chars();
            let mut c = chars.next();
            let mut n = chars.next();

            if !check_member_name_borders(&c.unwrap()) {
                return false;
            }
            loop {
                c = n;
                n = chars.next();
                if matches!(n, None) {
                    break;
                }
                if let Some(curr) = c {
                    if !check_member_name_corpus(&curr) {
                        return false;
                    }
                }
            }
            check_member_name_borders(&c.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        assert_eq!(check_member_name("fields"), true, "should've match");
        assert_eq!(check_member_name("fiel-ds"), true, "should've match");
        assert_eq!(check_member_name("fiel ds"), true, "should've match");
        assert_eq!(check_member_name("fiel_ds"), true, "should've match");
        assert_eq!(check_member_name("fiel2ds"), true, "should've match");
        assert_eq!(check_member_name("fielAds"), true, "should've match");
        assert_eq!(check_member_name("f"), true, "should've match");
    }

    #[test]
    fn too_short() {
        assert_eq!(check_member_name(""), false, "should'nt've match");
    }

    #[test]
    fn forbidden_begin_and_end_chars() {
        assert_eq!(check_member_name("-field"), false, "should'nt've match");
        assert_eq!(check_member_name("_field"), false, "should'nt've match");
        assert_eq!(check_member_name(" field"), false, "should'nt've match");
        assert_eq!(check_member_name("field-"), false, "should'nt've match");
        assert_eq!(check_member_name("field_"), false, "should'nt've match");
        assert_eq!(check_member_name("field "), false, "should'nt've match");
    }

    #[test]
    fn unicode() {
        assert_eq!(
            check_member_name("😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀"),
            true,
            "should've match"
        );
    }

    #[test]
    fn reserved_chars() {
        assert_eq!(check_member_name("fi+elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi,elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi.elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi[elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi]elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi!elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi'elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi#elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi$elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi%elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi&elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi\"elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi(elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi)elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi*elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi/elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi:elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi<elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi=elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi>elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi?elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi@elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi\\elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi^elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi`elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi{elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi}elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi|elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi~elds"), false, "should'nt've match");
        assert_eq!(check_member_name("fi\x7Felds"), false, "should'nt've match");
        assert_eq!(check_member_name("f\x00elds"), false, "should'nt've match");
        assert_eq!(
            check_member_name("fi\u{00}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{01}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{02}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{03}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{04}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{05}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{06}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{07}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{08}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{09}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{0A}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{0B}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{0C}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{0D}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{0F}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{10}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{11}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{12}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{13}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{14}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{15}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{16}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{17}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{18}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{19}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{1A}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{1B}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{1C}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{1D}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            check_member_name("fi\u{1F}elds"),
            false,
            "should'nt've match"
        );
    }
}
