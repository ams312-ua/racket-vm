use crate::{parsers::{DefaultParser, RParser, composed::FnArgs}, token::Token};
use chumsky::prelude::*;

/// Parser for function arguments.
/// 
/// Examples:
/// ```text
/// a b c
/// x y . rest
/// ```
#[derive(Clone, Copy)]
pub struct ArgsParser;

impl RParser for ArgsParser {
    type Output<'a> = FnArgs<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        text::ident()
            .padded()
            .repeated()
            .collect::<Vec<_>>()
            .then(
                just('.')
                    .padded()
                    .ignore_then(text::ident().padded())
                    .or_not(),
            ).map(|(named, variadic)| FnArgs {
                named,
                variadic,
            })
    }

    fn to_token<'a>(_src: Self::Output<'a>) -> Token<'a> {
        panic!("ArgsParser does not produce a Token directly");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn parses_named_only() {
        let p = ArgsParser::raw_parser();
        let res = p.parse("a b c").into_result();
        assert!(res.is_ok());
        let fa = res.unwrap();
        assert_eq!(fa.named, vec!["a", "b", "c"]);
        assert!(fa.variadic.is_none());
    }

    #[test]
    fn parses_variadic() {
        let p = ArgsParser::raw_parser();
        let res = p.parse("x y . rest").into_result();
        assert!(res.is_ok());
        let fa = res.unwrap();
        assert_eq!(fa.named, vec!["x", "y"]);
        assert_eq!(fa.variadic, Some("rest"));
    }

    #[test]
    fn parses_empty_args() {
        let p = ArgsParser::raw_parser();
        let res = p.parse("").into_result();
        assert!(res.is_ok());
        let fa = res.unwrap();
        assert!(fa.named.is_empty());
        assert!(fa.variadic.is_none());
    }

    #[test]
    fn allows_whitespace_and_newlines() {
        let p = ArgsParser::raw_parser();
        let res = p.parse("  a\n  b   .   rest").into_result();
        assert!(res.is_ok());
        let fa = res.unwrap();
        assert_eq!(fa.named, vec!["a", "b"]);
        assert_eq!(fa.variadic, Some("rest"));
    }

    #[test]
    fn rejects_dot_without_ident() {
        let p = ArgsParser::raw_parser();
        // trailing dot with no identifier should fail
        let res = p.parse("a b .").into_result();
        assert!(res.is_err());
    }
}
