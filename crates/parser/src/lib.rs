use crate::parsers::DefaultParser;
use crate::token::Token;
use chumsky::prelude::*;

pub mod parsers;
pub mod token;
pub mod util;

/*
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
}*/

/// Returns the parser to call to parse any kind of racket code.
pub fn parser<'a>() -> impl DefaultParser<'a, Vec<Token<'a>>> {
    use parsers::*;
    // Parsers must be recursive since they must be able to parse themselves
    recursive(|top| {
        let define = keywords::define::DefineParser::token_parser(recursive(|_| {
            choice((
                composed::function::FunctionParser::token_parser(top.clone()),
                top.clone(),
            ))
        }));

        let r#if = keywords::r#if::IfParser::token_parser(top.clone());

        let cond = keywords::cond::CondParser::token_parser(top.clone());

        let quoted = choice((
            quoted::cons::ConsParser::token_parser(top.clone()),
            quoted::list::ListParser::token_parser(top.clone())
        ));

        let composed = choice((
            composed::lambda::LambdaParser::token_parser(top.clone()),
            composed::call::CallParser::token_parser(top.clone()),
        ));

        let primitive_parser = primitives::AnyPrimitiveParser::token_parser().padded();

        choice((
            define,
            r#if,
            cond,
            quoted,
            composed,
            primitive_parser
        ))
    })
    .repeated()
    .collect::<Vec<_>>()
    .boxed()
}