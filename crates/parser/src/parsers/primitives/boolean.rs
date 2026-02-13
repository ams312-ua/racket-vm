use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;

/// Parser for boolean expressions.
///
/// Examples:
/// ```text
/// #t
/// #f
/// ```
#[derive(Clone, Copy)]
pub struct BooleanParser;

impl RParser for BooleanParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        choice((
            just("#true")
                .or(just("#T"))
                .or(just("#t"))
                .to(Primitive::Boolean(true)),

            just("#false")
                .or(just("#F"))
                .or(just("#f"))
                .to(Primitive::Boolean(false)),
        ))
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
    fn parses_true_variants() {
        let parser = BooleanParser::raw_parser();

        let r1 = parser.parse("#t").into_result();
        assert!(r1.is_ok());
        match r1.unwrap() {
            Primitive::Boolean(b) => assert!(b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }

        let r2 = parser.parse("#T").into_result();
        assert!(r2.is_ok());
        match r2.unwrap() {
            Primitive::Boolean(b) => assert!(b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }

        let r3 = parser.parse("#true").into_result();
        assert!(r3.is_ok());
        match r3.unwrap() {
            Primitive::Boolean(b) => assert!(b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }
    }

    #[test]
    fn parses_false_variants() {
        let parser = BooleanParser::raw_parser();

        let r1 = parser.parse("#f").into_result();
        assert!(r1.is_ok());
        match r1.unwrap() {
            Primitive::Boolean(b) => assert!(!b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }

        let r2 = parser.parse("#F").into_result();
        assert!(r2.is_ok());
        match r2.unwrap() {
            Primitive::Boolean(b) => assert!(!b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }

        let r3 = parser.parse("#false").into_result();
        assert!(r3.is_ok());
        match r3.unwrap() {
            Primitive::Boolean(b) => assert!(!b),
            other => panic!("expected Primitive::Boolean, got {:?}", other),
        }
    }

    #[test]
    fn rejects_similar_but_invalid() {
        let parser = BooleanParser::raw_parser();

        // `#trues` should not match the `#true` alternative
        let r = parser.parse("#trues").into_result();
        assert!(r.is_err());

        // plain `t` is invalid
        let r2 = parser.parse("t").into_result();
        assert!(r2.is_err());
    }

    #[test]
    fn token_parser_maps_to_boolean_token() {
        let parser = BooleanParser::token_parser();
        let res = parser.parse("#t").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Boolean(b)) => assert!(b),
            other => panic!("expected Token::Primitive::Boolean, got: {:?}", other),
        }
    }
}
