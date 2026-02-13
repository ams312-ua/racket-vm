use chumsky::prelude::*;
use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;

/// Parser for strings.
///
/// Parses quoted strings, handling escape sequences (e.g., `\"`, `\\`, `\n`).
///
/// Examples:
/// ```text
/// "hello"
/// "line\nbreak"
/// "escaped \"quote\""
/// ```
#[derive(Clone, Copy)]
pub struct StringParser;

impl RParser for StringParser {
    type Output<'a> = Primitive<'a>;
    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let escaped = just('\\')
            .then(choice((
                // Escapes are at https://docs.racket-lang.org/reference/reader.html#%28part._parse-string%29
                just('\\'),
                just('"'),
                just('a'),
                just('b'),
                just('t'),
                just('n'),
                just('v'),
                just('f'),
                just('r'),
                just('e'),
                just('\'')
            )))
            .ignored()
            .boxed();

        none_of("\\\"")
            .ignored()
            .or(escaped)
            .repeated()
            .to_slice()
            .delimited_by(just('"'), just('"'))
            .map(Primitive::String)
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
    fn parses_basic_string() {
        let parser = StringParser::raw_parser();
        let res = parser.parse("\"hello\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::String(ss) => assert_eq!(ss, "hello"),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn parses_empty_string() {
        let parser = StringParser::raw_parser();
        let res = parser.parse("\"\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::String(ss) => assert_eq!(ss, ""),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn preserves_escape_sequences_in_slice() {
        let parser = StringParser::raw_parser();
        // The parser returns a slice of the inner contents including escape sequences
        let res = parser.parse("\"line\\nbreak\\\"quote\\\"\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        // Expect the raw inner text with backslashes preserved
        match s {
            Primitive::String(ss) => assert_eq!(ss, "line\\nbreak\\\"quote\\\""),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn unicode_inside_string() {
        let parser = StringParser::raw_parser();
        let res = parser.parse("\"こんにちは\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::String(ss) => assert_eq!(ss, "こんにちは"),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn rejects_unclosed_string() {
        let parser = StringParser::raw_parser();
        let res = parser.parse("\"not closed").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn rejects_invalid_escape_sequence() {
        let parser = StringParser::raw_parser();
        // \x is not a supported escape in this parser
        let res = parser.parse("\"bad\\x\"").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_token_string() {
        let parser = StringParser::token_parser();
        let res = parser.parse("\"tok\"").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::String(s)) => assert_eq!(s, "tok"),
            other => panic!("expected Token::Primitive::String, got: {:?}", other),
        }
    }
}