use crate::parsers::DefaultParser;
use crate::token::Token;
use chumsky::prelude::*;

pub mod parsers;
pub mod token;
pub mod util;

/// Returns the parser to call to parse any kind of racket code.
pub fn parser<'a>() -> impl DefaultParser<'a, Vec<Token<'a>>> {
    use parsers::*;

    choice((
        keywords::define::DefineParser::token_parser(),
        keywords::r#if::IfParser::token_parser(),
        keywords::cond::CondParser::token_parser(),
        quoted::any_quoted::AnyQuotedParser::token_parser(),
        composed::any_composed::AnyComposedParser::token_parser(),
        primitives::AnyPrimitiveParser::token_parser(),
    )).repeated()
    .collect::<Vec<_>>()
}
