use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

#[derive(Clone, Copy)]
pub struct OctalParser;

impl RParser for OctalParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let oct = text::digits(8).repeated().to_slice();

        just("#o")
            .ignore_then(oct)
            .map(Primitive::Octal)
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
    fn parses_octal_digits() {
        let parser = OctalParser::raw_parser();
        let res = parser.parse("#o755").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Octal(ss) => assert_eq!(ss, "755"),
            other => panic!("expected Primitive::Octal, got {:?}", other),
        }
    }

    #[test]
    fn allows_empty_octal_after_prefix() {
        let parser = OctalParser::raw_parser();
        // `text::digits(8).repeated()` allows zero digits, so this should parse to an empty slice
        let res = parser.parse("#o").into_result();
        assert!(res.is_ok());
        let s = res.unwrap();
        match s {
            Primitive::Octal(ss) => assert_eq!(ss, ""),
            other => panic!("expected Primitive::Octal, got {:?}", other),
        }
    }

    #[test]
    fn rejects_invalid_octal_digit() {
        let parser = OctalParser::raw_parser();
        // '8' is not a valid octal digit
        let res = parser.parse("#o128").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_octal_token() {
        let parser = OctalParser::token_parser();
        let res = parser.parse("#o644").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Octal(s)) => assert_eq!(s, "644"),
            other => panic!("expected Token::Primitive::Octal, got: {:?}", other),
        }
    }
}