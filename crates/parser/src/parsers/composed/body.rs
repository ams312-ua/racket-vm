use chumsky::prelude::*;
use either::Either;

use crate::parsers::{RParser, RecursiveRParser, composed::{Composed, FnBody}, primitives::AnyPrimitiveParser, RecursiveParser, DefaultParser};
use crate::token::Token;

/// Parser for function bodies.
#[derive(Clone, Copy)]
pub struct BodyParser;

impl RecursiveRParser for BodyParser {
    type Output<'a> = FnBody<'a>;

    type RecursiveParserOutput<'a> = Token<'a>;

    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        choice((
            inner,
            AnyPrimitiveParser::token_parser(),
        ))
        .separated_by(just(' '))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|item| FnBody {
            content: item
        })
    }

    fn to_token<'a>(_src: Self::Output<'a>) -> Token<'a> {
        panic!("BodyParser does not produce Tokens")
    }
}
