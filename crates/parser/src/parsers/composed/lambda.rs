use chumsky::prelude::*;

use crate::{
	parsers::{
		DefaultParser, RParser, RecursiveParser, RecursiveRParser,
		composed::{Composed, args::ArgsParser, body::BodyParser, Function},
	},
	token::Token,
};

pub struct LambdaParser;

impl RecursiveRParser for LambdaParser {
	type Output<'a> = Composed<'a>;

	type RecursiveParserOutput<'a> = Token<'a>;

	fn raw_parser<'a, 'b>(
		inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
	) -> impl DefaultParser<'a, Self::Output<'a>> {
		let args = ArgsParser::raw_parser()
			.padded()
			.delimited_by(just('('), just(')'));

		just("lambda")
			.padded()
			.ignore_then(args)
			.then(BodyParser::raw_parser(inner).padded())
			.delimited_by(just('(').padded(), just(')').padded())
			.map(|(args, body)| {
				Composed::Function(Function {
					name: None,
					args,
					body: Box::new(body),
				})
			})
	}

	fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
		Token::Composed(src)
	}
}
