use crate::parsers::{DefaultParser, RParser, primitives::StringParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

/// Parser for byte literals.
/// 
/// Examples:
/// ```text
/// #"a124"
/// #"advb"
/// #"ab"
/// ```
#[derive(Clone, Copy)]
pub struct BytesParser;

impl RParser for BytesParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        // Expect a '#' followed by a string primitive; convert to bytes primitive
        just('#').ignore_then(
            StringParser::raw_parser().map(|p| match p {
                Primitive::String(s) => Primitive::Bytes(s.as_bytes()),
                _ => unreachable!("expected Primitive::String from StringParser"),
            })
        )
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
    fn parses_basic_bytes() {
        let parser = BytesParser::raw_parser();
        let res = parser.parse("#\"ab\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Bytes(b) => assert_eq!(b, b"ab" as &[u8]),
            other => panic!("expected Primitive::Bytes, got {:?}", other),
        }
    }

    #[test]
    fn parses_empty_bytes() {
        let parser = BytesParser::raw_parser();
        let res = parser.parse("#\"\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Bytes(b) => assert_eq!(b, b"" as &[u8]),
            other => panic!("expected Primitive::Bytes, got {:?}", other),
        }
    }

    #[test]
    fn preserves_escape_sequences() {
        let parser = BytesParser::raw_parser();
        let res = parser.parse("#\"line\\nbreak\\\"\"").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Bytes(b) => assert_eq!(b, b"line\\nbreak\\\"" as &[u8]),
            other => panic!("expected Primitive::Bytes, got {:?}", other),
        }
    }

    #[test]
    fn rejects_unclosed_string() {
        let parser = BytesParser::raw_parser();
        let res = parser.parse("#\"not closed").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_bytes_token() {
        let parser = BytesParser::token_parser();
        let res = parser.parse("#\"hi\"").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Bytes(b)) => assert_eq!(b, b"hi" as &[u8]),
            other => panic!("expected Token::Primitive::Bytes, got: {:?}", other),
        }
    }
}
