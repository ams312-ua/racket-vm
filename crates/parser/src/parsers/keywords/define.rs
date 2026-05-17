use chumsky::prelude::*;

use crate::{
    parsers::{
        DefaultParser, RParser, RecursiveParser, RecursiveRParser, composed::{Composed, Function, any_composed::AnyComposedParser}, keywords::Keyword, primitives::{AnyPrimitiveParser, IdentifierParser, Primitive}, quoted::any_quoted::AnyQuotedParser
    },
    token::Token,
};

pub struct DefineParser;

impl RecursiveRParser for DefineParser {
    type Output<'a> = Keyword<'a>;

    type RecursiveParserOutput<'a> = Token<'a>;

    fn raw_parser<'a, 'b>(
        value: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>,
    ) -> impl DefaultParser<'a, Self::Output<'a>> {
        just("define")
            .padded()
            .ignore_then(IdentifierParser::raw_parser().padded().or_not())
            .then(value)
            .delimited_by(just('(').padded(), just(')').padded())
            .try_map(|(mut name, value), span| {
                if let None = name {
                    // name can be optional in the case of defining a function with the syntax (define (fn args) body)
                    // in that case, take the name from the function definition instead.
                    if let Token::Composed(Composed::Function(Function { name: Some(fn_name), .. })) = &value {
                        name = Some(Primitive::Ident(*fn_name));
                    } else {
                        return Err(Rich::custom(span, "define requires a name for the variable or function being defined"));
                    }
                }

                let Some(Primitive::Ident(name)) = name else {
                    unreachable!("IdentifierParser should only produce Identifier primitives")
                };

                Ok(Keyword::Define {
                    name,
                    value: Box::new(value)
                })
            })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Keyword(src)
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use chumsky::Parser;

    use super::*;

    fn parse_ok(input: &str) -> Keyword<'_> {
        todo!()
        /*DefineParser::raw_parser()
            .parse(input)
            .into_result()
            .expect("define form should parse")*/
    }

    fn parse_err(input: &str) {
        todo!()
        /*let res = DefineParser::raw_parser().parse(input).into_result();
        assert!(res.is_err(), "expected parse error for: {input}");*/
    }

    #[test]
    fn parses_define_with_identifier_and_primitive() {
        let parsed = parse_ok("(define x 42)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "x");
                match value.as_ref() {
                    Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 42),
                    other => panic!("expected integer primitive, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_define_function_shorthand_and_uses_function_name() {
        let parsed = parse_ok("(define (add x y) (+ x y))");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "add");
                match value.as_ref() {
                    Token::Composed(Composed::Function(function)) => {
                        assert_eq!(function.name, Some("add"));
                        assert_eq!(function.args.named, vec!["x", "y"]);
                    }
                    other => panic!("expected composed function, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_define_with_quoted_value() {
        let parsed = parse_ok("(define xs '(1 2 3))");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "xs");
                match value.as_ref() {
                    Token::Quoted(_) => {}
                    other => panic!("expected quoted token, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn rejects_define_without_name_when_value_is_not_function() {
        parse_err("(define 42)");
    }

    #[test]
    fn parses_define_with_tree_value() {
        let parsed = parse_ok("(define sum (+ 1 2))");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "sum");
                match value.as_ref() {
                    Token::Composed(Composed::Tree(tree)) => {
                        assert_eq!(tree.root.as_ref(), &Token::Primitive(Primitive::Ident("+")));
                        assert_eq!(tree.leaves.len(), 2);
                    }
                    other => panic!("expected composed tree, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_define_with_boolean_value() {
        let parsed = parse_ok("(define enabled #true)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "enabled");
                match value.as_ref() {
                    Token::Primitive(Primitive::Boolean(v)) => assert!(*v),
                    other => panic!("expected boolean primitive, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_define_with_special_identifier_name() {
        let parsed = parse_ok("(define + 1)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "+");
                match value.as_ref() {
                    Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 1),
                    other => panic!("expected integer primitive, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_define_with_heavy_whitespace() {
        let parsed = parse_ok("(  define\n  my-val\n   42\n)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "my-val");
                match value.as_ref() {
                    Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 42),
                    other => panic!("expected integer primitive, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_shorthand_function_with_no_args() {
        let parsed = parse_ok("(define (answer) 42)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "answer");
                match value.as_ref() {
                    Token::Composed(Composed::Function(function)) => {
                        assert_eq!(function.name, Some("answer"));
                        assert!(function.args.named.is_empty());
                    }
                    other => panic!("expected function token, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_shorthand_function_with_variadic_args() {
        let parsed = parse_ok("(define (collect x . rest) x)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "collect");
                match value.as_ref() {
                    Token::Composed(Composed::Function(function)) => {
                        assert_eq!(function.name, Some("collect"));
                        assert_eq!(function.args.named, vec!["x"]);
                        assert_eq!(function.args.variadic, Some("rest"));
                    }
                    other => panic!("expected function token, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn parses_shorthand_function_with_multiple_body_expressions() {
        let parsed = parse_ok("(define (dup x) x x)");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "dup");
                match value.as_ref() {
                    Token::Composed(Composed::Function(function)) => {
                        assert_eq!(function.body.content.len(), 2);
                    }
                    other => panic!("expected function token, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn keeps_explicit_name_when_value_is_named_function() {
        let parsed = parse_ok("(define alias (inner x) (+ x 1))");

        match parsed {
            Keyword::Define { name, value } => {
                assert_eq!(name, "alias");
                match value.as_ref() {
                    Token::Composed(Composed::Function(function)) => {
                        assert_eq!(function.name, Some("inner"));
                    }
                    other => panic!("expected function token, got {:?}", other),
                }
            },
            _ => panic!("expected define keyword, got something else"),
        }
    }

    #[test]
    fn rejects_missing_outer_parentheses() {
        parse_err("define x 1");
    }

    #[test]
    fn rejects_non_define_keyword() {
        parse_err("(def x 1)");
    }

    #[test]
    fn rejects_missing_value() {
        parse_err("(define x)");
    }

    #[test]
    fn rejects_too_many_value_expressions() {
        parse_err("(define x 1 2)");
    }

    #[test]
    fn rejects_empty_define_form() {
        parse_err("(define)");
    }

    #[test]
    fn rejects_shorthand_function_without_function_name() {
        parse_err("(define () 1)");
    }

    #[test]
    fn rejects_name_omitted_with_non_function_composed_value() {
        parse_err("(define (+ 1 2))");
    }

    #[test]
    fn rejects_name_omitted_with_quoted_value() {
        parse_err("(define '(1 2 3))");
    }
}