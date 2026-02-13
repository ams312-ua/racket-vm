use chumsky::prelude::*;
use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::{IntegerParser, Primitive};
use crate::token::Token;

/// Parser for single-precision (f32) numeric literals.
///
/// Examples:
/// ```text
/// 0.5
/// -0.125
/// ```
#[derive(Clone, Copy)]
pub struct FloatParser;

impl RParser for FloatParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        // Parse the fractional part of a float. We use digits cause 0001 is valid
        let digits = text::digits(10).to_slice();

        // Parse the decimal point and fractional digits
        let frac = just('.').ignore_then(digits);
        
        // Combine integer parser with the ones we just defined for floats
        IntegerParser::raw_parser()
            .then(frac)
            .map(|(int, s)| {
                // int here is Primitive::Integer — extract value
                let int_val = match int {
                    Primitive::Integer(v) => v,
                    _ => unreachable!("expected Primitive::Integer from IntegerParser"),
                };
                let v: f32 = format!("{}.{s}", int_val).parse().unwrap();
                Primitive::Float(v)
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
    fn parses_basic_float() {
        let parser = FloatParser::raw_parser();
        let res = parser.parse("0.5").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Float(v) => assert_eq!(v, 0.5_f32),
            other => panic!("expected Primitive::Float, got {:?}", other),
        }
    }

    #[test]
    fn parses_negative_float() {
        let parser = FloatParser::raw_parser();
        let res = parser.parse("-1.125").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Float(v) => assert_eq!(v, -1.125_f32),
            other => panic!("expected Primitive::Float, got {:?}", other),
        }
    }

    #[test]
    fn parses_with_leading_plus() {
        let parser = FloatParser::raw_parser();
        let res = parser.parse("+3.14").into_result();
        assert!(res.is_ok());
        match res.unwrap() {
            Primitive::Float(v) => assert_eq!(v, 3.14_f32),
            other => panic!("expected Primitive::Float, got {:?}", other),
        }
    }

    #[test]
    fn rejects_missing_fractional_digits() {
        let parser = FloatParser::raw_parser();
        // `1.` should be rejected because the fractional digits are required
        let res = parser.parse("1.").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn rejects_non_digit_fraction() {
        let parser = FloatParser::raw_parser();
        let res = parser.parse("1.a").into_result();
        assert!(res.is_err());
    }

    #[test]
    fn token_parser_maps_to_float_token() {
        let parser = FloatParser::token_parser();
        let res = parser.parse("-2.5").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Primitive(Primitive::Float(f)) => assert_eq!(f, -2.5_f32),
            other => panic!("expected Token::Primitive::Float, got: {:?}", other),
        }
    }
}