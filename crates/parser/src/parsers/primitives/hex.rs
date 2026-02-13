use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

#[derive(Clone, Copy)]
pub struct HexParser;

impl RParser for HexParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let hex = text::digits(16).repeated().to_slice();

        just("#x")
            .ignore_then(hex)
            .map(Primitive::Hex)
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
    fn parses_hex_digits() {
        let parser = HexParser::raw_parser();
        let res = parser.parse("#x7f").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Hex(ss) => assert_eq!(ss, "7f"),
            other => panic!("expected Primitive::Hex, got {:?}", other),
        }
    }

    #[test]
    fn allows_empty_hex_after_prefix() {
        let parser = HexParser::raw_parser();
        let res = parser.parse("#x").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Hex(ss) => assert_eq!(ss, ""),
            other => panic!("expected Primitive::Hex, got {:?}", other),
        }
    }

    #[test]
    fn rejects_invalid_hex_digit() {
        let parser = HexParser::raw_parser();
        let res = parser.parse("#xg1").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_hex_token() {
        let parser = HexParser::token_parser();
        let res = parser.parse("#x0A").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Hex(s)) => assert_eq!(s, "0A"),
            other => panic!("expected Token::Primitive::Hex, got: {:?}", other),
        }
    }
}