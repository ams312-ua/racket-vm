use either::Either;

use crate::{parsers::{DefaultParser, RParser, RecursiveParser, RecursiveRParser, primitives::AnyPrimitiveParser, quoted::Quoted}, token::Token};
use chumsky::prelude::*;

pub struct ConsParser;

impl RecursiveRParser for ConsParser {
    type Output<'a> = Quoted<'a>;

    type RecursiveParserOutput<'a> = Token<'a>;

    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        let primitive_or_quoted = inner.clone()
            .or(AnyPrimitiveParser::token_parser());

        // Separator: one or more spaces, dot, one or more spaces
        let dot_separator = just(' ')
            .repeated()
            .at_least(1)
            .then_ignore(just('.'))
            .then_ignore(just(' ').repeated().at_least(1));

        primitive_or_quoted.clone()
            .then_ignore(dot_separator)
            .then(primitive_or_quoted)
            .padded()
            .delimited_by(just("'("), just(')'))
            .map(|(token, token2)| {
                    Quoted::Cons {
                        left: Box::new(token),
                        right: Box::new(token2),
                    }
                })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Quoted(src)
    }
}
