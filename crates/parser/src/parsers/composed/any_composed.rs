use chumsky::prelude::{choice, recursive};

use crate::{parsers::{DefaultParser, RParser, RecursiveRParser, composed::{Composed, call::CallParser, function::FunctionParser, lambda::LambdaParser}}, token::Token};

/// Parser for any composed expression.
/// This parser can parse any composed structure defined in the language.
/// Examples:
/// ```text
/// (function-name arg1 arg2)
/// ```
#[derive(Clone, Copy)]
pub struct AnyComposedParser;

impl RParser for AnyComposedParser {
    type Output<'a> = Token<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        recursive(|p| {
            choice((
                LambdaParser::token_parser(p.clone()),
                FunctionParser::token_parser(p.clone()),
                CallParser::token_parser(p.clone()),
            ))
        })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn parses_function_with_no_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(foo ) 42").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert_eq!(func.name, Some("foo"));
                assert!(func.args.named.is_empty());
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn parses_function_with_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(add x y ) (+ x y)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert_eq!(func.name, Some("add"));
                assert_eq!(func.args.named, vec!["x", "y"]);
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn parses_function_with_variadic() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(list-all x . rest ) x").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert_eq!(func.name, Some("list-all"));
                assert_eq!(func.args.named, vec!["x"]);
                assert_eq!(func.args.variadic, Some("rest"));
            }
            other => panic!("expected Function with variadic, got {:?}", other),
        }
    }

    #[test]
    fn function_with_multiple_exprs() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(list-all x . rest ) 2 x").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert_eq!(func.name, Some("list-all"));
                assert!(func.args.named.len() == 1);
                assert!(func.args.variadic == Some("rest"));
                assert_eq!(func.body.content.len(), 2);

                println!("{:?}", func);
            }
            other => panic!("expected Function with multiple exprs, got {:?}", other),
        }
    }

    #[test]
    fn rejects_invalid_cons_missing_dot() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(42 100)").into_result();
        assert!(res.is_err(), "should reject cons without dot");
    }

    #[test]
    fn rejects_unclosed_paren() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(42 . 100").into_result();
        assert!(res.is_err(), "should reject unclosed paren");
    }

    #[test]
    fn parses_function_with_multiple_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(multiply a b c ) (* a b c)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert_eq!(func.name, Some("multiply"));
                assert_eq!(func.args.named, vec!["a", "b", "c"]);
            }
            other => panic!("expected Function with 3 args, got {:?}", other),
        }
    }

    #[test]
    fn parses_simple_call() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(print 42)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "print");
                assert_eq!(tree.leaves.len(), 1);
            }
            other => panic!("expected Tree (call), got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_multiple_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(max 10 20 30)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "max");
                assert_eq!(tree.leaves.len(), 3);
            }
            other => panic!("expected Tree with multiple args, got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_strings() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(concat \"hello\" \"world\")").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "concat");
                assert_eq!(tree.leaves.len(), 2);
            }
            other => panic!("expected Tree with strings, got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_whitespace() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(  format   1   2   )").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "format");
                assert_eq!(tree.leaves.len(), 2);
            }
            other => panic!("expected Tree with whitespace, got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_nested_calls() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(outer (inner 42))").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "outer");
                assert_eq!(tree.leaves.len(), 1);
            }
            other => panic!("expected Tree with nested call, got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_mixed_primitives() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(func 42 \"text\" #true)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "func");
                assert_eq!(tree.leaves.len(), 3);
            }
            other => panic!("expected Tree with mixed primitives, got {:?}", other),
        }
    }

    #[test]
    fn parses_call_with_no_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(get-value)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Tree(tree)) => {
                assert_eq!(tree.root, "get-value");
                assert!(tree.leaves.is_empty());
            }
            other => panic!("expected Tree with no args, got {:?}", other),
        }
    }

    #[test]
    fn parses_lambda_no_args_single_body() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda () 42)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert!(func.name.is_none());
                assert!(func.args.named.is_empty());
                assert!(func.args.variadic.is_none());
                assert_eq!(func.body.content.len(), 1);
            }
            other => panic!("expected lambda function, got {:?}", other),
        }
    }

    #[test]
    fn parses_lambda_with_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda (x y) (+ x y))").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert!(func.name.is_none());
                assert_eq!(func.args.named, vec!["x", "y"]);
                assert!(func.args.variadic.is_none());
            }
            other => panic!("expected lambda function, got {:?}", other),
        }
    }

    #[test]
    fn parses_lambda_variadic_args() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda (x . rest) x)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert!(func.name.is_none());
                assert_eq!(func.args.named, vec!["x"]);
                assert_eq!(func.args.variadic, Some("rest"));
            }
            other => panic!("expected lambda function, got {:?}", other),
        }
    }

    #[test]
    fn parses_lambda_multiple_body_exprs() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda (x) 1 2)").into_result();
        assert!(res.is_ok());
        let token = res.unwrap();
        match token {
            Token::Composed(Composed::Function(func)) => {
                assert!(func.name.is_none());
                assert_eq!(func.args.named, vec!["x"]);
                assert_eq!(func.body.content.len(), 2);
            }
            other => panic!("expected lambda function, got {:?}", other),
        }
    }

    #[test]
    fn rejects_lambda_missing_body() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda (x))").into_result();
        assert!(res.is_err(), "should reject lambda with no body");
    }

    #[test]
    fn rejects_lambda_missing_args_parens() {
        let parser = AnyComposedParser::token_parser();
        let res = parser.parse("(lambda x 1)").into_result();
        assert!(res.is_err(), "should reject lambda without args parens");
    }
}