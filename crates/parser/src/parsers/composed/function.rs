use crate::{parsers::{DefaultParser, RParser, RecursiveParser, RecursiveRParser, composed::{Composed, args::ArgsParser, body::BodyParser}, primitives::{IdentifierParser, Primitive}}, token::Token};
use chumsky::prelude::*;
use crate::parsers::composed::Function;

pub struct FunctionParser;

impl RecursiveRParser for FunctionParser {
    type Output<'a> = Composed<'a>;

    type RecursiveParserOutput<'a> = Composed<'a>;

    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> 
    {
        let fn_identifier = IdentifierParser::raw_parser().map(|res| {
            let Primitive::Ident(name) = res else {
                unreachable!("IdentifierParser should only produce Identifier primitives")
            };

            name
        });
        let name_args = fn_identifier.padded()
            .then(ArgsParser::raw_parser().padded())
            .delimited_by(just('('), just(')'));

        name_args.padded().then(BodyParser::raw_parser(inner))
            .map(|((name, args), body)| Composed::Function(Function {
                name: Some(name),
                args,
                body: Box::new(body),
            }))
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Composed(src)
    }
}
