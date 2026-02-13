use chumsky::{error::Rich, prelude::*};
use either::Either;

use crate::{parsers::{DefaultParser, RParser, RecursiveRParser, composed::{Composed, Tree}, primitives::{AnyPrimitiveParser, IdentifierParser, Primitive}}, token::{KEYWORDS, Token}};

pub struct CallParser;

impl RecursiveRParser for CallParser {
    type Output<'a> = Composed<'a>;

    type RecursiveParserOutput<'a> = Composed<'a>;

    fn raw_parser<'a, 'b>(
        inner: crate::parsers::RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        IdentifierParser::raw_parser()
            .try_map(|p, span| {
                let Primitive::Ident(i) = p else {
                    unreachable!("IdentifierParser should only produce Identifier primitives")
                };

                if KEYWORDS.contains(&i) {
                    Err(Rich::custom(span, "reserved keyword cannot be called"))
                } else {
                    Ok(i)
                }
            })
            .then(
                AnyPrimitiveParser::token_parser()
                    .map(Either::Left)
                    .or(inner.map(Either::Right))
                    .padded()
                    .repeated()
                    .collect::<Vec<_>>()
            )
            .delimited_by(just("(").padded(), just(")").padded())
            .map(|(func_name, args)| Composed::Tree(Tree {
                root: func_name,
                leaves: args,
            }))
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Composed(src)
    }
}