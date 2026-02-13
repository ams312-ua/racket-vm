use chumsky::prelude::*;
use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;

/// Parser for integer numeric literals.
///
/// Recognizes an optional sign and a sequence of digits.
///
/// Examples:
/// ```text
/// 0
/// 12345
/// -42
/// ```
#[derive(Clone, Copy)]
pub struct IntegerParser;

impl RParser for IntegerParser {
    type Output<'a> = Primitive<'a>;
    
    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        // Parse a single int on base 10 to string slice
        one_of("+-")
            .or_not()
            .then(text::int(10))
            .map(|(sign, i)| {
                let v: i64 = format!("{}{}", sign.unwrap_or('+'), i).parse().unwrap();
                Primitive::Integer(v)
            })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        // We know the src will always be a valid digit because of how we implemented the parser,
        // so this can never fail
        Token::Primitive(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;
    use crate::token::Token;

    #[test]
    fn parses_zero() {
        let parser = IntegerParser::raw_parser();
        let res = parser.parse("0").into_result();
        assert!(res.is_ok());
        let v = res.unwrap();
        match v {
            Primitive::Integer(i) => assert_eq!(i, 0),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }
    }

    #[test]
    fn parses_positive_and_negative() {
        let parser = IntegerParser::raw_parser();
        let res_pos = parser.parse("123").into_result();
        assert!(res_pos.is_ok());
        match res_pos.unwrap() {
            Primitive::Integer(i) => assert_eq!(i, 123),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }

        let res_neg = parser.parse("-42").into_result();
        assert!(res_neg.is_ok());
        match res_neg.unwrap() {
            Primitive::Integer(i) => assert_eq!(i, -42),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }
    }

    #[test]
    fn parses_leading_plus() {
        let parser = IntegerParser::raw_parser();
        let res = parser.parse("+7").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Integer(i) => assert_eq!(i, 7),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }
    }

    #[test]
    fn rejects_minus_only() {
        let parser = IntegerParser::raw_parser();
        let res = parser.parse("-").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_integer_token() {
        let parser = IntegerParser::token_parser();
        let res = parser.parse("-100").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Integer(i)) => assert_eq!(i, -100),
            other => panic!("expected Token::Primitive::Integer, got: {:?}", other),
        }
    }
}
