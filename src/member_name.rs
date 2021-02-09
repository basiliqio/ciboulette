use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MEMBER_NAME_REGEX: Regex = Regex::new(r#"
	^(?x)																														# Disable whitespace consideration in regex
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
	]*?																																		# For every other characters, if any
	
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
