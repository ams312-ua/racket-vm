use chumsky::{error::Rich, prelude::*};
use either::Either;

use crate::{
    parsers::{
        DefaultParser, RParser, RecursiveParser, RecursiveRParser,
        composed::{Composed, Tree},
        primitives::{AnyPrimitiveParser, IdentifierParser, Primitive},
    },
    token::{KEYWORDS, Token},
};

pub struct CallParser;

impl RecursiveRParser for CallParser {
    type Output<'a> = Composed<'a>;

    type RecursiveParserOutput<'a> = Token<'a>;

    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>,
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        /*let ident_parser = IdentifierParser::token_parser().try_map(|p, span| {
            let Token::Primitive(Primitive::Ident(i)) = p else {
                unreachable!("IdentifierParser should only produce Identifier primitives")
            };

            if KEYWORDS.contains(&i) {
                Err(Rich::custom(span, "reserved keyword cannot be called"))
            } else {
                Ok(p)
            }
        });*/

        // Root can be either an identifier or another call (for higher-order functions)
        let parser = IdentifierParser::token_parser().or(inner.clone()).try_map(|v, s| {
            let Token::Primitive(Primitive::Ident(i)) = v else {
                return Err(Rich::custom(s, "root of a call must be an identifier or another call"));
            };

            if KEYWORDS.contains(&i) {
                return Err(Rich::custom(s, "reserved keyword cannot be called"))
            }

            // A call can only have an identifier or another call as root, anything else is a syntax error
            match v {
                Token::Primitive(Primitive::Ident(_)) | Token::Composed(_) => Ok(v),
                _ => Err(Rich::custom(
                    s,
                    "invalid function call: root of a call must be an identifier or another call",
                )),
            }
        });

        parser
            .clone()
            .then(
                AnyPrimitiveParser::token_parser()
                    .or(inner)
                    .padded()
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .delimited_by(just("(").padded(), just(")").padded())
            .try_map(|(func_name, args), span| {
                // leaves cannot be a function that is not a lambda
                /*for arg in &args {
                    if let Token::Composed(Composed::Function(f)) = arg {
                        if f.name.is_some() {
                            return Err(Rich::custom(
                                span,
                                "function call cannot have a function that is not a lambda",
                            ));
                        }
                    }
                }*/

                Ok(Composed::Tree(Tree {
                    root: Box::new(func_name),
                    leaves: args,
                }))
            })

        /*IdentifierParser::raw_parser()
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
                .or(inner)
                .padded()
                .repeated()
                .collect::<Vec<_>>()
        )
        .delimited_by(just("(").padded(), just(")").padded())
        .map(|(func_name, args)| Composed::Tree(Tree {
            root: func_name,
            leaves: args,
        }))*/
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Composed(src)
    }
}
