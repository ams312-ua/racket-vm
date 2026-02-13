use crate::parsers::{DefaultParser, RParser};
use crate::parsers::primitives::*;
use crate::token::Token;
use chumsky::prelude::*;

/// Base parser that can parse any primitive token type, so it can be used with composites.
/// 
/// This parser tries to parse the input as any of the known token types,
/// returning the first successful parse as a `Token` variant.
#[derive(Clone, Copy)]
pub struct AnyPrimitiveParser;

impl RParser for AnyPrimitiveParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        choice((
            // #rx (regex) is specific, prefer it early
            RegexParser::raw_parser(),
            // Byte string: #"..."
            BytesParser::raw_parser(),
            // Character: #\...
            CharacterParser::raw_parser(),
            // Numeric/radix prefixes: hex, octal, binary
            HexParser::raw_parser(),
            OctalParser::raw_parser(),
            BinaryParser::raw_parser(),
            // Booleans (#t, #f, #true, #false)
            BooleanParser::raw_parser(),
            // Plain string literals
            StringParser::raw_parser(),
            // Numbers: prefer double (with exponent) -> float (with dot) -> integer
            DoubleParser::raw_parser(),
            FloatParser::raw_parser(),
            IntegerParser::raw_parser(),
            // Identifiers: last resort as they're the most general pattern.
            // Can start with letters, underscores, or special chars (+, -, *, etc),
            // so must come after more specific patterns like numbers and prefixed tokens.
            IdentifierParser::raw_parser(),
        )).boxed()
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

    fn parse_one(input: &str) -> Token<'_> {
        let parser = AnyPrimitiveParser::token_parser();
        parser.parse(input).into_result().expect("should parse token")
    }

    #[test]
    fn parses_string() {
        let t = parse_one("\"hello\"");
        match t {
            Token::Primitive(Primitive::String(s)) => assert_eq!(s, "hello"),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn parses_bytes() {
        let t = parse_one("#\"ab\"");
        match t {
            Token::Primitive(Primitive::Bytes(b)) => assert_eq!(b, b"ab" as &[u8]),
            other => panic!("expected Primitive::Bytes, got {:?}", other),
        }
    }

    #[test]
    fn parses_regex() {
        let t = parse_one("#rx\"a|b\"");
        match t {
            Token::Primitive(Primitive::Regex(s)) => assert_eq!(s, "a|b"),
            other => panic!("expected Primitive::Regex, got {:?}", other),
        }
    }

    #[test]
    fn parses_character_variants() {
        let t1 = parse_one("#\\a");
        match t1 {
            Token::Primitive(Primitive::Character(s)) => assert_eq!(s, "#\\a"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }

        let t2 = parse_one("#\\space");
        match t2 {
            Token::Primitive(Primitive::Character(s)) => assert_eq!(s, "#\\space"),
            other => panic!("expected Primitive::Character, got {:?}", other),
        }
    }

    #[test]
    fn parses_radix_tokens() {
        match parse_one("#x7f") {
            Token::Primitive(Primitive::Hex(s)) => assert_eq!(s, "7f"),
            other => panic!("expected Primitive::Hex, got {:?}", other),
        }

        match parse_one("#o755") {
            Token::Primitive(Primitive::Octal(s)) => assert_eq!(s, "755"),
            other => panic!("expected Primitive::Octal, got {:?}", other),
        }

        match parse_one("#b1010") {
            Token::Primitive(Primitive::Binary(s)) => assert_eq!(s, "1010"),
            other => panic!("expected Primitive::Binary, got {:?}", other),
        }
    }

    #[test]
    fn parses_booleans() {
        match parse_one("#t") {
            Token::Primitive(Primitive::Boolean(b)) => assert!(b),
            other => panic!("expected Primitive::Boolean true, got {:?}", other),
        }

        match parse_one("#false") {
            Token::Primitive(Primitive::Boolean(b)) => assert!(!b),
            other => panic!("expected Primitive::Boolean false, got {:?}", other),
        }
    }

    #[test]
    fn parses_numbers() {
        match parse_one("42") {
            Token::Primitive(Primitive::Integer(i)) => assert_eq!(i, 42),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }

        match parse_one("-7") {
            Token::Primitive(Primitive::Integer(i)) => assert_eq!(i, -7),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }

        match parse_one("3.14") {
            Token::Primitive(Primitive::Float(f)) => assert_eq!(f, 3.14_f32),
            other => panic!("expected Primitive::Float, got {:?}", other),
        }

        match parse_one("6.02e+23") {
            Token::Primitive(Primitive::Double(d)) => assert_eq!(d, 6.02e23_f64),
            other => panic!("expected Primitive::Double, got {:?}", other),
        }
    }

    #[test]
    fn preserves_escape_sequences_in_string_slice() {
        let t = parse_one("\"line\\nbreak\\\"quote\\\"\"");
        match t {
            Token::Primitive(Primitive::String(s)) => assert_eq!(s, "line\\nbreak\\\"quote\\\""),
            other => panic!("expected Primitive::String, got {:?}", other),
        }
    }

    #[test]
    fn padded_parses_with_whitespace() {
        //let t = parse_one("   123   ");
        let parser = AnyPrimitiveParser::token_parser().padded();
        let res = parser.parse("   123   ").into_result();
        assert!(res.is_ok());
        let t = res.unwrap();
        match t {
            Token::Primitive(Primitive::Integer(i)) => assert_eq!(i, 123),
            other => panic!("expected Primitive::Integer, got {:?}", other),
        }
    }
}
