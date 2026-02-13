use crate::parsers::primitives::{FloatParser, IntegerParser, Primitive};
use crate::parsers::{DefaultParser, RParser};
use crate::token::Token;
use chumsky::prelude::*;
use either::Either;

/// Parser for double-precision (f64) numeric literals.
///
/// Examples:
/// ```text
/// 6.02e+23
/// 3e-2
/// ```
#[derive(Clone, Copy)]
pub struct DoubleParser;

impl RParser for DoubleParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        // Same as float parser but for exponents
        let digits = text::digits(10).to_slice();

        // Parse the exponent part
        let exp = just('e')
            .or(just('E'))
            .then(one_of("+-"))
            .then(digits)
            .map(|((_, sign), digits)| {
                format!("e{sign}{digits}")
            });

        // Start with the float parser to get the double parser
        FloatParser::raw_parser().map(Either::Left)
            .or(IntegerParser::raw_parser().map(Either::Right))
            .then(exp)
            .map(|(decimal, exp)| {
                // decimal is either Primitive::Float or Primitive::Integer
                let decimal_str = match decimal {
                    Either::Left(Primitive::Float(f)) => f.to_string(),
                    Either::Right(Primitive::Integer(i)) => i.to_string(),
                    _ => unreachable!("unexpected primitive for decimal"),
                };
                let v: f64 = format!("{}{}", decimal_str, exp).parse().unwrap();
                Primitive::Double(v)
            })
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
    fn parses_double_with_exponent() {
        let parser = DoubleParser::raw_parser();
        let res = parser.parse("6.02e+23").into_result();
        assert!(res.is_ok());
        let v = res.unwrap();
        match v {
            Primitive::Double(f) => assert_eq!(f, 6.02e23_f64),
            other => panic!("expected Primitive::Double, got {:?}", other),
        }
    }

    #[test]
    fn parses_uppercase_e_and_negative_exp() {
        let parser = DoubleParser::raw_parser();
        let res = parser.parse("1E-3").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Double(f) => assert_eq!(f, 1e-3_f64),
            other => panic!("expected Primitive::Double, got {:?}", other),
        }
    }

    #[test]
    fn rejects_missing_exponent_digits() {
        let parser = DoubleParser::raw_parser();
        // e+ with no digits should be rejected
        let res = parser.parse("2.0e+").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_double_token() {
        let parser = DoubleParser::token_parser();
        let res = parser.parse("3.0e-2").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Double(d)) => assert_eq!(d, 3.0e-2_f64),
            other => panic!("expected Token::Primitive::Double, got: {:?}", other),
        }
    }
}
