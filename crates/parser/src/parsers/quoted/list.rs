use crate::{parsers::{DefaultParser, RParser, RecursiveParser, RecursiveRParser, primitives::{AnyPrimitiveParser, Primitive}, quoted::Quoted}, token::Token};
use chumsky::{error::Rich, prelude::*};
use either::Either;

pub struct ListParser;

impl RecursiveRParser for ListParser {
    type Output<'a> = Quoted<'a>;

    type RecursiveParserOutput<'a> = Token<'a>;

    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        inner.clone()
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .padded()
            .delimited_by(just("'("), just(')'))
            .try_map(|items, span| {
                let has_dot_ident = items.iter().any(|item| {
                    // Because of the way idents are, they have to support a single dot, so it gets parsed.
                    // But we want to avoid dots as standalone items in lists, as that would be ambiguous with cons cells.
                    matches!(item, Token::Primitive(Primitive::Ident(".")))
                });

                if has_dot_ident {
                    Err(Rich::custom(span, "dot not allowed in list as standalone item"))
                } else {
                    Ok(Quoted::List(items))
                }
            })
            
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Quoted(src)
    }
}