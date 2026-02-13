use crate::parsers::DefaultParser;
use crate::token::Token;
use chumsky::prelude::*;

pub mod parsers;
pub mod token;
pub mod util;

/// Returns the parser to call to parse any kind of racket code.
pub fn parser<'a>() -> impl DefaultParser<'a, Vec<Token<'a>>> {
    chumsky::prelude::todo()
}
