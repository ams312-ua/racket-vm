use chumsky::prelude::*;

use crate::{
	parsers::{
		DefaultParser, RParser,
		composed::{Composed, any_composed::AnyComposedParser},
		keywords::Keyword,
		primitives::{AnyPrimitiveParser, Primitive},
		quoted::any_quoted::AnyQuotedParser,
	},
	token::Token,
};

pub struct IfParser;

impl RParser for IfParser {
	type Output<'a> = Keyword<'a>;

	fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
		recursive(|if_expr| {
			let atom = choice((
				AnyPrimitiveParser::token_parser(),
				AnyComposedParser::token_parser(),
				AnyQuotedParser::token_parser(),
			));

			let value = choice((if_expr.map(Token::Keyword), atom)).padded();

			just("if")
				.padded()
				.ignore_then(value.clone())
				.then(value.clone())
				.then(value)
				.delimited_by(just('(').padded(), just(')').padded())
				.map(|((condition, then_branch), else_branch)| Keyword::If {
					condition: Box::new(condition),
					then_branch: Box::new(then_branch),
					else_branch: Box::new(else_branch),
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
		IfParser::raw_parser()
			.parse(input)
			.into_result()
			.expect("if form should parse")
	}

	fn parse_err(input: &str) {
		let res = IfParser::raw_parser().parse(input).into_result();
		assert!(res.is_err(), "expected parse error for: {input}");
	}

	#[test]
	fn parses_if_with_then_and_else_primitives() {
		let parsed = parse_ok("(if #true 1 0)");

		match parsed {
			Keyword::If {
				condition,
				then_branch,
				else_branch,
			} => {
				match condition.as_ref() {
					Token::Primitive(Primitive::Boolean(v)) => assert!(*v),
					other => panic!("expected boolean condition, got {:?}", other),
				}

				match then_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 1),
					other => panic!("expected integer then branch, got {:?}", other),
				}

				match else_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 0),
					other => panic!("expected integer else branch, got {:?}", other),
				}
			}
			_ => panic!("expected if keyword, got something else"),
		}
	}

	#[test]
	fn rejects_if_without_else() {
		parse_err("(if #t 42)");
	}

	#[test]
	fn parses_if_with_composed_and_quoted_branches() {
		let parsed = parse_ok("(if #t (+ 1 2) '(3 4))");

		match parsed {
			Keyword::If {
				then_branch,
				else_branch,
				..
			} => {
				match then_branch.as_ref() {
					Token::Composed(Composed::Tree(tree)) => {
						assert_eq!(tree.root.as_ref(), &Token::Primitive(Primitive::Ident("+")));
						assert_eq!(tree.leaves.len(), 2);
					}
					other => panic!("expected composed tree then branch, got {:?}", other),
				}

				match else_branch.as_ref() {
					Token::Quoted(_) => {}
					other => panic!("expected quoted else branch, got {:?}", other),
				}
			}
			_ => panic!("expected if keyword, got something else"),
		}
	}

	#[test]
	fn parses_nested_if_expression() {
		let parsed = parse_ok("(if (if #t #f #t) 1 0)");

		match parsed {
			Keyword::If { condition, .. } => match condition.as_ref() {
				Token::Keyword(Keyword::If { .. }) => {}
				other => panic!("expected nested if condition, got {:?}", other),
			},
			_ => panic!("expected if keyword, got something else"),
		}
	}

	#[test]
	fn parses_if_with_heavy_whitespace() {
		let parsed = parse_ok("(  if\n #true\n  1\n  0\n)");

		match parsed {
			Keyword::If {
				then_branch,
				else_branch,
				..
			} => {
				match then_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 1),
					other => panic!("expected integer then branch, got {:?}", other),
				}

				match else_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 0),
					other => panic!("expected integer else branch, got {:?}", other),
				}
			}
			_ => panic!("expected if keyword, got something else"),
		}
	}

	#[test]
	fn rejects_missing_outer_parentheses() {
		parse_err("if #t 1 0");
	}

	#[test]
	fn rejects_non_if_keyword() {
		parse_err("(iff #t 1 0)");
	}

	#[test]
	fn rejects_missing_condition_and_then() {
		parse_err("(if)");
	}

	#[test]
	fn rejects_missing_then_branch() {
		parse_err("(if #t)");
	}

	#[test]
	fn rejects_too_many_expressions() {
		parse_err("(if #t 1 0 2)");
	}
}
