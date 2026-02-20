use chumsky::prelude::*;
use either::Either;

use crate::{
    parsers::{
        DefaultParser, RParser, RecursiveRParser, composed::{Composed, FnArgs, FnBody, Function, any_composed::AnyComposedParser, function::FunctionParser, lambda::LambdaParser}, keywords::Keyword, primitives::{AnyPrimitiveParser, IdentifierParser, Primitive}, quoted::any_quoted::AnyQuotedParser
    },
    token::Token,
};

pub struct DefineParser;

impl RParser for DefineParser {
    type Output<'a> = Keyword<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        let value = choice((
            AnyPrimitiveParser::token_parser(),
            AnyComposedParser::token_parser(),
            AnyQuotedParser::token_parser()
        ));

        just("define")
            .padded()
            .ignore_then(IdentifierParser::raw_parser().padded())
            .then(value)
            .delimited_by(just('(').padded(), just(')').padded())
            .map(|(name, value)| {
                let Primitive::Ident(name) = name else {
                    unreachable!("IdentifierParser should only produce Identifier primitives")
                };
                Keyword::Define {
                    name,
                    value: Box::new(value)
                }
            })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Keyword(src)
    }
}