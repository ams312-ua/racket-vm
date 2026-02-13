use chumsky::prelude::*;

use crate::{parsers::{DefaultParser, RParser, RecursiveRParser, quoted::{Quoted, cons::ConsParser, list::ListParser}}, token::Token};

pub struct AnyQuotedParser;

impl RParser for AnyQuotedParser {
    type Output<'a> = Quoted<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        recursive(|s| {
            choice((
                ConsParser::raw_parser(s.clone()),
                ListParser::raw_parser(s)
            ))
        })
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Quoted(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;
    use either::Either::{Left, Right};

    use crate::parsers::primitives::Primitive;
    use crate::token::Token;

    fn parse_one(input: &str) -> Token {
        let parser = AnyQuotedParser::token_parser();
        parser.parse(input).into_result().expect("should parse quoted token")
    }

    fn parse_err(input: &str) {
        let parser = AnyQuotedParser::token_parser();
        let res = parser.parse(input).into_result();
        assert!(res.is_err(), "expected parse error for: {input}");
    }

    #[test]
    fn parses_simple_list() {
        let token = parse_one("'(1 2 3)");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 3);
                match &items[0] {
                    Left(Token::Primitive(Primitive::Integer(i))) => assert_eq!(*i, 1),
                    other => panic!("expected integer 1, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_cons_pair() {
        let token = parse_one("'(1 . 2)");
        match token {
            Token::Quoted(Quoted::Cons { left, right }) => {
                match left.as_ref() {
                    Left(Token::Primitive(Primitive::Integer(i))) => assert_eq!(*i, 1),
                    other => panic!("expected left integer, got {:?}", other),
                }
                match right.as_ref() {
                    Left(Token::Primitive(Primitive::Integer(i))) => assert_eq!(*i, 2),
                    other => panic!("expected right integer, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::Cons, got {:?}", other),
        }
    }

    #[test]
    fn parses_nested_quoted_list() {
        let token = parse_one("'(1 '(2 3))");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 2);
                match &items[1] {
                    Right(Quoted::List(inner)) => {
                        assert_eq!(inner.len(), 2);
                    }
                    other => panic!("expected nested quoted list, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_list_with_identifiers_and_booleans() {
        let token = parse_one("'(foo #t bar #false)");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 4);
                match &items[0] {
                    Left(Token::Primitive(Primitive::Ident(s))) => assert_eq!(*s, "foo"),
                    other => panic!("expected ident foo, got {:?}", other),
                }
                match &items[1] {
                    Left(Token::Primitive(Primitive::Boolean(b))) => assert!(*b),
                    other => panic!("expected boolean true, got {:?}", other),
                }
                match &items[3] {
                    Left(Token::Primitive(Primitive::Boolean(b))) => assert!(!*b),
                    other => panic!("expected boolean false, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_list_with_extra_spaces_between_items() {
        let token = parse_one("'(1  2   3)");
        match token {
            Token::Quoted(Quoted::List(items)) => assert_eq!(items.len(), 3),
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_list_with_nested_cons() {
        let token = parse_one("'(1 '(2 . 3))");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 2);
                match &items[1] {
                    Right(Quoted::Cons { .. }) => {}
                    other => panic!("expected nested quoted cons, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_list_with_string_bytes_regex_and_char() {
        let token = parse_one("'(\"hi\" #\"ab\" #rx\"a|b\" #\\space)");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 4);
                match &items[0] {
                    Left(Token::Primitive(Primitive::String(s))) => assert_eq!(*s, "hi"),
                    other => panic!("expected string hi, got {:?}", other),
                }
                match &items[1] {
                    Left(Token::Primitive(Primitive::Bytes(b))) => assert_eq!(*b, b"ab" as &[u8]),
                    other => panic!("expected bytes ab, got {:?}", other),
                }
                match &items[2] {
                    Left(Token::Primitive(Primitive::Regex(s))) => assert_eq!(*s, "a|b"),
                    other => panic!("expected regex a|b, got {:?}", other),
                }
                match &items[3] {
                    Left(Token::Primitive(Primitive::Character(s))) => assert_eq!(*s, "#\\space"),
                    other => panic!("expected character #\\space, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_list_with_numeric_variants() {
        let token = parse_one("'(#x7f #o10 #b1010 -42 3.14 6.02e+23)");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 6);
                match &items[0] {
                    Left(Token::Primitive(Primitive::Hex(s))) => assert_eq!(*s, "7f"),
                    other => panic!("expected hex 7f, got {:?}", other),
                }
                match &items[1] {
                    Left(Token::Primitive(Primitive::Octal(s))) => assert_eq!(*s, "10"),
                    other => panic!("expected octal 10, got {:?}", other),
                }
                match &items[2] {
                    Left(Token::Primitive(Primitive::Binary(s))) => assert_eq!(*s, "1010"),
                    other => panic!("expected binary 1010, got {:?}", other),
                }
                match &items[3] {
                    Left(Token::Primitive(Primitive::Integer(i))) => assert_eq!(*i, -42),
                    other => panic!("expected integer -42, got {:?}", other),
                }
                match &items[4] {
                    Left(Token::Primitive(Primitive::Float(f))) => assert_eq!(*f, 3.14_f32),
                    other => panic!("expected float 3.14, got {:?}", other),
                }
                match &items[5] {
                    Left(Token::Primitive(Primitive::Double(d))) => assert_eq!(*d, 6.02e23_f64),
                    other => panic!("expected double 6.02e23, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_cons_with_quoted_list_on_right() {
        let token = parse_one("'(1 . '(2 3))");
        match token {
            Token::Quoted(Quoted::Cons { right, .. }) => match right.as_ref() {
                Right(Quoted::List(items)) => assert_eq!(items.len(), 2),
                other => panic!("expected quoted list on right, got {:?}", other),
            },
            other => panic!("expected Quoted::Cons, got {:?}", other),
        }
    }

    #[test]
    fn parses_cons_with_quoted_list_on_left() {
        let token = parse_one("'('(1 2) . 3)");
        match token {
            Token::Quoted(Quoted::Cons { left, .. }) => match left.as_ref() {
                Right(Quoted::List(items)) => assert_eq!(items.len(), 2),
                other => panic!("expected quoted list on left, got {:?}", other),
            },
            other => panic!("expected Quoted::Cons, got {:?}", other),
        }
    }

    #[test]
    fn parses_cons_with_mixed_primitives() {
        let token = parse_one("'(#t . \"hi\")");
        match token {
            Token::Quoted(Quoted::Cons { left, right }) => {
                match left.as_ref() {
                    Left(Token::Primitive(Primitive::Boolean(b))) => assert!(*b),
                    other => panic!("expected boolean true on left, got {:?}", other),
                }
                match right.as_ref() {
                    Left(Token::Primitive(Primitive::String(s))) => assert_eq!(*s, "hi"),
                    other => panic!("expected string hi on right, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::Cons, got {:?}", other),
        }
    }

    #[test]
    fn parses_nested_quoted_depth_three() {
        let token = parse_one("'('(1 '(2 '(3 4))))");
        match token {
            Token::Quoted(Quoted::List(items)) => {
                assert_eq!(items.len(), 1);
                match &items[0] {
                    Right(Quoted::List(inner)) => {
                        assert_eq!(inner.len(), 2);
                    }
                    other => panic!("expected inner quoted list, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::List, got {:?}", other),
        }
    }

    #[test]
    fn parses_cons_with_nested_cons_on_both_sides() {
        let token = parse_one("'('(1 . 2) . '(3 . 4))");
        match token {
            Token::Quoted(Quoted::Cons { left, right }) => {
                match left.as_ref() {
                    Right(Quoted::Cons { .. }) => {}
                    other => panic!("expected quoted cons on left, got {:?}", other),
                }
                match right.as_ref() {
                    Right(Quoted::Cons { .. }) => {}
                    other => panic!("expected quoted cons on right, got {:?}", other),
                }
            }
            other => panic!("expected Quoted::Cons, got {:?}", other),
        }
    }

    #[test]
    fn rejects_unclosed_list() {
        parse_err("'(1 2");
    }

    #[test]
    fn rejects_empty_list() {
        parse_err("'()");
    }

    #[test]
    fn rejects_trailing_space_before_paren_close() {
        parse_err("'(1 2 )");
    }

    #[test]
    fn rejects_tabs_or_newlines_as_separators() {
        parse_err("'(1\t2)");
        parse_err("'(1\n2)");
    }

    #[test]
    fn rejects_space_between_quote_and_paren() {
        parse_err("' (1 2)");
    }

    #[test]
    fn rejects_missing_spaces_around_dot() {
        parse_err("'(1. 2)");
        parse_err("'(1 .2)");
    }

    #[test]
    fn rejects_extra_dot_or_items_in_cons() {
        parse_err("'(1 . 2 3)");
        parse_err("'(1 . . 2)");
        parse_err("'(1 . )");
        parse_err("'(. 1)");
    }

    #[test]
    fn rejects_non_quoted_list_on_cons_side() {
        parse_err("'(1 . (2))");
    }

    #[test]
    fn rejects_leading_space_in_list() {
        parse_err("'( 1 2)");
    }
}
