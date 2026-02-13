use crate::parsers::composed::Composed;
use crate::parsers::primitives::AnyPrimitiveParser;
use crate::parsers::{DefaultParser, RParser, RecursiveParser, RecursiveRParser};
use crate::token::Token;
use chumsky::prelude::*;
use either::Either;

#[derive(Debug, Clone)]
pub struct ConsParser;

impl RecursiveRParser for ConsParser
{
    type Output<'a> = Composed<'a>;
    type RecursiveParserOutput<'a> = Composed<'a>;
    
    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        let composed_or_token = inner.clone().map(Either::Right)
            .or(AnyPrimitiveParser::token_parser().map(Either::Left));

        // (a .          b        )
        // -> ()
        // .
        // lo que sea . lo que sea

        // Pareja 1 elem -> Lista
        // Forma especial / Funcion diferencia

        composed_or_token.clone()
            .then_ignore(just('.').padded())
            .then(composed_or_token)
            .padded()
            .delimited_by(just('('), just(')'))
            .map(|(token, token2)| {
                    Composed::Cons {
                        head: Box::new(token),
                        tail: Box::new(token2),
                    }
                })
                
    }
    
    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Composed(src)
    }
}
