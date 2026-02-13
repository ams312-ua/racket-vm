use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

#[derive(Clone, Copy)]
pub struct BinaryParser;

impl RParser for BinaryParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let bin = text::digits(2).repeated().to_slice();

        just("#b").ignore_then(bin).map(Primitive::Binary)
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
    fn parses_binary_digits() {
        let parser = BinaryParser::raw_parser();
        let res = parser.parse("#b1010").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Binary(ss) => assert_eq!(ss, "1010"),
            other => panic!("expected Primitive::Binary, got {:?}", other),
        }
    }

    #[test]
    fn allows_empty_binary_after_prefix() {
        let parser = BinaryParser::raw_parser();
        let res = parser.parse("#b").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Binary(ss) => assert_eq!(ss, ""),
            other => panic!("expected Primitive::Binary, got {:?}", other),
        }
    }

    #[test]
    fn rejects_invalid_binary_digit() {
        let parser = BinaryParser::raw_parser();
        let res = parser.parse("#b102").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_binary_token() {
        let parser = BinaryParser::token_parser();
        let res = parser.parse("#b1101").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Binary(s)) => assert_eq!(s, "1101"),
            other => panic!("expected Token::Primitive::Binary, got: {:?}", other),
        }
    }
}