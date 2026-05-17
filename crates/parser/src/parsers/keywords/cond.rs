use chumsky::{error::Rich, prelude::*};
use either::Either;

use crate::{
	parsers::{
		DefaultParser, RParser, RecursiveParser, RecursiveRParser, composed::any_composed::AnyComposedParser, keywords::{Keyword, r#if::IfParser}, primitives::AnyPrimitiveParser, quoted::any_quoted::AnyQuotedParser
	},
	token::Token,
};

pub struct CondParser;

impl RecursiveRParser for CondParser {
	type Output<'a> = Keyword<'a>;

	type RecursiveParserOutput<'a> = Token<'a>;

	fn raw_parser<'a, 'b>(
		value: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>,
	) -> impl DefaultParser<'a, Self::Output<'a>> {
		/*recursive(|cond_expr| {
			let atom = choice((
				IfParser::raw_parser().map(Token::Keyword),
				AnyPrimitiveParser::token_parser(),
				AnyComposedParser::token_parser(),
				AnyQuotedParser::token_parser(),
			));

			let value = choice((cond_expr.map(Token::Keyword), atom)).padded();

			
		})*/

		let branch = value
				.clone()
				.then(value.clone())
				.delimited_by(just('(').padded(), just(')').padded())
				.map(Either::Left);

			let else_branch = just("else")
				.padded()
				.ignore_then(value.clone())
				.delimited_by(just('(').padded(), just(')').padded())
				.map(Either::Right);

			just("cond")
				.padded()
				.ignore_then(choice((else_branch, branch)).padded().repeated().at_least(1).collect::<Vec<_>>())
				.delimited_by(just('(').padded(), just(')').padded())
				.try_map(|clauses, span| {
					let mut branches = Vec::new();
					let mut else_clause = None;

					for clause in clauses {
						match clause {
							Either::Left((condition, expression)) => {
								if else_clause.is_some() {
									return Err(Rich::custom(span, "else clause must be last in cond"));
								}

								branches.push((condition, expression));
							}
							Either::Right(value) => {
								if else_clause.is_some() {
									return Err(Rich::custom(span, "cond cannot contain more than one else clause"));
								}

								else_clause = Some(Box::new(value));
							}
						}
					}

					Ok(Keyword::Cond {
						branches,
						else_branch: else_clause,
					})
				})
	}

	fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
		Token::Keyword(src)
	}
}

#[cfg(test)]
mod tests {
	use chumsky::Parser;

	use super::*;
	use crate::parsers::{composed::Composed, primitives::Primitive};

	fn parse_ok(input: &str) -> Keyword<'_> {
		todo!()
		/*CondParser::raw_parser()
			.parse(input)
			.into_result()
			.expect("cond form should parse")*/
	}

	fn parse_err(input: &str) {
		todo!()
		/*let res = CondParser::raw_parser().parse(input).into_result();
		assert!(res.is_err(), "expected parse error for: {input}");*/
	}

	#[test]
	fn parses_cond_with_branches_and_else() {
		let parsed = parse_ok("(cond (#false 1) (#true 2) (else 3))");

		match parsed {
			Keyword::Cond {
				branches,
				else_branch,
			} => {
				assert_eq!(branches.len(), 2);

				match &branches[0].0 {
					Token::Primitive(Primitive::Boolean(v)) => assert!(!*v),
					other => panic!("expected boolean condition in branch 0, got {:?}", other),
				}

				match &branches[1].1 {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 2),
					other => panic!("expected integer expression in branch 1, got {:?}", other),
				}

				let Some(else_branch) = else_branch else {
					panic!("expected else branch");
				};

				match else_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 3),
					other => panic!("expected integer else branch, got {:?}", other),
				}
			}
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn parses_cond_without_else() {
		let parsed = parse_ok("(cond (#t 1) (#f 2))");

		match parsed {
			Keyword::Cond {
				branches,
				else_branch,
			} => {
				assert_eq!(branches.len(), 2);
				assert!(else_branch.is_none());
			}
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn parses_cond_with_only_else() {
		let parsed = parse_ok("(cond (else 42))");

		match parsed {
			Keyword::Cond {
				branches,
				else_branch,
			} => {
				assert!(branches.is_empty());

				let Some(else_branch) = else_branch else {
					panic!("expected else branch");
				};

				match else_branch.as_ref() {
					Token::Primitive(Primitive::Integer(v)) => assert_eq!(*v, 42),
					other => panic!("expected integer else branch, got {:?}", other),
				}
			}
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn parses_cond_with_composed_quoted_and_if() {
		let parsed = parse_ok("(cond ((> x 0) (+ x 1)) ((if #t #f #t) '(1 2)) (else (if #t 9 8)))");

		match parsed {
			Keyword::Cond {
				branches,
				else_branch,
			} => {
				assert_eq!(branches.len(), 2);

				match &branches[0].1 {
					Token::Composed(Composed::Tree(tree)) => {
						assert_eq!(tree.root.as_ref(), &Token::Primitive(Primitive::Ident("+")));
						assert_eq!(tree.leaves.len(), 2);
					}
					other => panic!("expected composed expression in branch 0, got {:?}", other),
				}

				match &branches[1].0 {
					Token::Keyword(Keyword::If { .. }) => {}
					other => panic!("expected if keyword as branch condition, got {:?}", other),
				}

				let Some(else_branch) = else_branch else {
					panic!("expected else branch");
				};

				match else_branch.as_ref() {
					Token::Keyword(Keyword::If { .. }) => {}
					other => panic!("expected if keyword as else branch, got {:?}", other),
				}
			}
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn parses_nested_cond_expression() {
		let parsed = parse_ok("(cond ((cond (else #t)) 1) (else 0))");

		match parsed {
			Keyword::Cond { branches, .. } => match &branches[0].0 {
				Token::Keyword(Keyword::Cond { .. }) => {}
				other => panic!("expected nested cond condition, got {:?}", other),
			},
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn parses_cond_with_heavy_whitespace() {
		let parsed = parse_ok("(  cond\n   (#t 1)\n   (else 0)\n)");

		match parsed {
			Keyword::Cond {
				branches,
				else_branch,
			} => {
				assert_eq!(branches.len(), 1);
				assert!(else_branch.is_some());
			}
			_ => panic!("expected cond keyword, got something else"),
		}
	}

	#[test]
	fn rejects_missing_outer_parentheses() {
		parse_err("cond (#t 1) (else 0)");
	}

	#[test]
	fn rejects_non_cond_keyword() {
		parse_err("(conde (#t 1) (else 0))");
	}

	#[test]
	fn rejects_empty_cond_form() {
		parse_err("(cond)");
	}

	#[test]
	fn rejects_branch_missing_expression() {
		parse_err("(cond (#t) (else 0))");
	}

	#[test]
	fn rejects_else_missing_expression() {
		parse_err("(cond (else))");
	}

	#[test]
	fn rejects_else_not_last() {
		parse_err("(cond (else 0) (#t 1))");
	}

	#[test]
	fn rejects_duplicate_else_clause() {
		parse_err("(cond (#t 1) (else 2) (else 3))");
	}

	#[test]
	fn rejects_branch_with_too_many_expressions() {
		parse_err("(cond (#t 1 2) (else 0))");
	}
}
