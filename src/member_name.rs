use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MEMBER_NAME_REGEX: Regex = Regex::new(r#"(?x)																		# Disable whitespace consideration in regex
	^																																# The case of only 1 char
	[																																
		a-z 																																# Accept lowercase letters
		A-Z 																																# Accept uppercase letters
		0-9 																																# Accept digits
		\U{0080}-\U{10FFFF}																													# Accept any unicode characters from 0x0080
		&&																															# Intersect with the following
		[^																																# Substract the following
			\s																																# Substract whitespaces
			_																																# Substract underscores
			\U{002D}																														# Substract hyphens
			[\U{002B}\U{002C}\U{002E}\U{005B}\U{005D}\U{0021}-\U{002A}\U{002F}\U{003A}-\U{0040}\U{005C}\U{005E}\U{0060}\U{007B}-\U{007F}]	# Substract reserved chars (https://jsonapi.org/format/#document-member-names-reserved-characters)
		]
	]{1}
	$
	|
	^
	[
		a-z 																																# Accept lowercase letters
		A-Z 																																# Accept uppercase letters
		0-9 																																# Accept digits
		\U{0080}-\U{10FFFF}																													# Accept any unicode characters from 0x0080
		&&																															# Intersect with the following
		[^																																# Substract the following
			\s																																# Substract whitespaces
			_																																# Substract underscores
			\U{002D}																														# Substract hyphens
			[\U{002B}\U{002C}\U{002E}\U{005B}\U{005D}\U{0021}-\U{002A}\U{002F}\U{003A}-\U{0040}\U{005C}\U{005E}\U{0060}\U{007B}-\U{007F}]	# Substract reserved chars (https://jsonapi.org/format/#document-member-names-reserved-characters)
		]
	]{1}																																	# Only for first character



	[
		a-z																																	# Accept lowercase letters
		A-Z																																	# Accept uppercase letters
		0-9																																	# Accept digits
		\U{0020}																															# Accept spaces
		_																																	# Accept underscores
		\U{002D}																															# Accept hyphens
		\U{0080}-\U{10FFFF}																													# Accept any unicode characters from 0x0080
		&&																															# Intersect with the following
		[^																																# Substract the following
			[\U{002B}\U{002C}\U{002E}\U{005B}\U{005D}\U{0021}-\U{002A}\U{002F}\U{003A}-\U{0040}\U{005C}\U{005E}\U{0060}\U{007B}-\U{007F}]	# Substract reserved chars (https://jsonapi.org/format/#document-member-names-reserved-characters)
		]
	]*																																		# For every other characters, if any
	
	[
		a-z 																																# Accept lowercase letters
		A-Z 																																# Accept uppercase letters
		0-9 																																# Accept digits
		\U{0080}-\U{10FFFF}																													# Accept any unicode characters from 0x0080
		&&																															# Intersect with the following
		[^																																# Substract the following
			\s																																# Substract whitespaces
			_																																# Substract underscores
			\U{002D}																														# Substract hyphens
			[\U{002B}\U{002C}\U{002E}\U{005B}\U{005D}\U{0021}-\U{002A}\U{002F}\U{003A}-\U{0040}\U{005C}\U{005E}\U{0060}\U{007B}-\U{007F}]	# Substract reserved chars (https://jsonapi.org/format/#document-member-names-reserved-characters)
		]
	]{1}																																	# Only for last character
	$
	"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fields"),
            true,
            "should've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fiel-ds"),
            true,
            "should've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fiel ds"),
            true,
            "should've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fiel_ds"),
            true,
            "should've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fiel2ds"),
            true,
            "should've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fielAds"),
            true,
            "should've match"
        );
        assert_eq!(MEMBER_NAME_REGEX.is_match("f"), true, "should've match");
    }

    #[test]
    fn too_short() {
        assert_eq!(MEMBER_NAME_REGEX.is_match(""), false, "should'nt've match");
    }

    #[test]
    fn forbidden_begin_and_end_chars() {
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("-field"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("_field"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match(" field"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("field-"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("field_"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("field "),
            false,
            "should'nt've match"
        );
    }

    #[test]
    fn unicode() {
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀"),
            true,
            "should've match"
        );
    }

    #[test]
    fn reserved_chars() {
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi+elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi,elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi.elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi[elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi]elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi!elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi'elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi#elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi$elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi%elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi&elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\"elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi(elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi)elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi*elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi/elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi:elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi<elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi=elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi>elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi?elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi@elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\\elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi^elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi`elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi{elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi|elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi~elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\x7Felds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("f\x00elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{00}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{01}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{02}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{03}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{04}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{05}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{06}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{07}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{08}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{09}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{0A}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{0B}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{0C}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{0D}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{0F}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{10}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{11}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{12}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{13}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{14}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{15}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{16}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{17}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{18}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{19}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{1A}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{1B}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{1C}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{1D}elds"),
            false,
            "should'nt've match"
        );
        assert_eq!(
            MEMBER_NAME_REGEX.is_match("fi\u{1F}elds"),
            false,
            "should'nt've match"
        );
    }
}
