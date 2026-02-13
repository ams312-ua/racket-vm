use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

/// Parser for characters.
///
/// Examples:
/// ```text
/// #\a
/// #\tab
/// #\space
/// ```
#[derive(Clone, Copy)]
pub struct CharacterParser;

impl RParser for CharacterParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let digit_8 = any().filter(|c: &char| c.is_digit(8))
            .repeated()
            .exactly(3)
            .to_slice();

        let digit_16_4 = any().filter(|c: &char| c.is_digit(16))
            .repeated()
            .at_least(1)
            .at_most(4)
            .to_slice();

        let digit_16_8 = any().filter(|c: &char| c.is_digit(16))
            .repeated()
            .at_least(1)
            .at_most(8)
            .to_slice();

        let single_char = any().to_slice();

        just(r"#\")
            .then(choice((
                just("null"),
                just("nul"),
                just("backspace"),
                just("tab"),
                just("newline"),
                just("linefeed"),
                just("vtab"),
                just("page"),
                just("return"),
                just("space"),
                just("rubout"),
                digit_8,
                just("u").ignore_then(digit_16_4),
                just("U").ignore_then(digit_16_8),
                single_char
            )))
            .to_slice()
            .map(Primitive::Character)
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Primitive(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;
    use crate::token::Token;

    #[test]
    fn parses_simple_character() {
        let parser = CharacterParser::raw_parser();
        let res = parser.parse("#\\a").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Character(ss) => assert_eq!(ss, "#\\a"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }
    }

    #[test]
    fn parses_named_and_whitespace_names() {
        let parser = CharacterParser::raw_parser();
        let tab = parser.parse("#\\tab").into_result();
        assert!(tab.is_ok());
        match tab.unwrap() {
            Primitive::Character(ss) => assert_eq!(ss, "#\\tab"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }

        let space = parser.parse("#\\space").into_result();
        assert!(space.is_ok());
        match space.unwrap() {
            Primitive::Character(ss) => assert_eq!(ss, "#\\space"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }

        let newline = parser.parse("#\\newline").into_result();
        assert!(newline.is_ok());
        match newline.unwrap() {
            Primitive::Character(ss) => assert_eq!(ss, "#\\newline"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }
    }

    #[test]
    fn parses_octal_and_unicode_codes() {
        let parser = CharacterParser::raw_parser();
        // octal exactly 3 digits
        let res = parser.parse("#\\123").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Character(ss) => assert_eq!(ss, "#\\123"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }

        // \u with 1-4 hex digits
        let res_u = parser.parse("#\\u1fA").into_result();
        assert!(res_u.is_ok());
        match res_u.unwrap() {
            Primitive::Character(ss) => assert_eq!(ss, "#\\u1fA"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }

        // \U with 1-8 hex digits
    let res_cap = parser.parse("#\\U0001F600").into_result();
    assert!(res_cap.is_ok());
    match res_cap.unwrap() {
        Primitive::Character(ss) => assert_eq!(ss, "#\\U0001F600"),
        other => panic!("expected Primitive::Character, got {:?}", other),
    }
    }

    #[test]
    fn rejects_incomplete_or_invalid_forms() {
    let parser = CharacterParser::raw_parser();
    // just the prefix should be rejected
    let res = parser.parse("#\\").into_result();
    assert!(res.is_err());

        // octal must be exactly 3 digits
    let res_oct_short = parser.parse("#\\12").into_result();
    assert!(res_oct_short.is_err());

        // invalid hex digit after u
        let res_bad_u = parser.parse("#\\uG").into_result();
        assert!(res_bad_u.is_err());
    }

    #[test]
    fn token_parser_maps_to_character_token() {
        let parser = CharacterParser::token_parser();
        let res = parser.parse("#\\space").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Character(s)) => assert_eq!(s, "#\\space"),
            other => panic!("expected Token::Primitive::Character, got: {:?}", other),
        }
    }
}
