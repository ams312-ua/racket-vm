use crate::parsers::{DefaultParser, RParser, primitives::StringParser};
use crate::parsers::primitives::Primitive;
use crate::token::Token;
use chumsky::prelude::*;


/// Parser for byte literals.
///
/// Examples:
/// ```text
/// #rx"a|b"
/// #rx"^c(a|d)+r$"
/// ```
#[derive(Clone, Copy)]
pub struct RegexParser;

impl RParser for RegexParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        // Consume the `#rx` prefix and return the inner string slice produced by
        // `StringParser::raw_parser()` (which already returns the inner slice).
        just("#rx").ignore_then(
            StringParser::raw_parser().map(|p| match p {
                Primitive::String(s) => Primitive::Regex(s),
                _ => unreachable!("expected Primitive::String from StringParser"),
            })
        )
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Primitive(src)
    }
}
