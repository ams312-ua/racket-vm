pub mod composed;
pub mod primitives;
pub mod quoted;
pub mod keywords;

use chumsky::Parser;
use chumsky::prelude::Recursive;
use chumsky::recursive::Direct;

use chumsky::error::Rich;
use chumsky::extra;
use crate::token::Token;

type FullError<'a> = extra::Err<Rich<'a, char>>;

pub trait DefaultParser<'src, O>: Parser<'src, &'src str, O, FullError<'src>> + Clone {}
impl<'src, O, F> DefaultParser<'src, O> for F
where
    F: Parser<'src, &'src str, O, FullError<'src>> + Clone
{}

/// Racket parser base trait.
///
/// Defines an interface to be able to combine parsers with other parsers to avoid code duplication
/// and easier fixes.
///
/// Implementors may only override [`RParser::raw_parser`] to implement parser behaviour, and
/// [`RParser::to_token`] to implement conversion from parser output to [`Token`].
///
/// A default implementation of [`RParser::token_parser`] is provided for standard conversions.
pub trait RParser {
    type Output<'a>;
    /// Creates a parser that resolves into a [Token] variant
    fn token_parser<'a>() -> impl DefaultParser<'a, Token<'a>> {
        // Raw parser + to_token, just that is enough
        Self::raw_parser().map(Self::to_token)
    }

    /// Creates a parser that resolves into a default string slice result.
    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>>;

    /// Converts the result of calling [`Parser::parse`] on the parser returned by
    /// [`RParser::raw_parser`].
    ///
    /// This function assumes the input is the one returned by the parser provided by
    /// [`RParser::raw_parser`]. Calling it with other inputs may result in crashes.
    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a>;
}

/// Type alias for recursive parsers used in [`RecursiveRParser`].
type RecursiveParser<'src, 'b, O> = Recursive<Direct<'src, 'b, &'src str, O, FullError<'src>>>;

/// Recursive parser variant.
///
/// Similar to `RParser` but allows passing a set of inner parsers (for example a handle
/// created with `chumsky::recursive`) so recursive productions can be expressed without
/// construction-time recursion.
pub trait RecursiveRParser {
    /// The output type of the parser.
    type Output<'a>;
    /// The output type of the inner recursive parser.
    type RecursiveParserOutput<'a>;

    /// Creates a parser that resolves into a [Token] variant using the provided inner parsers.
    fn token_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Token<'a>> {
        Self::raw_parser(inner).map(Self::to_token)
    }

    /// Creates a parser that resolves into the parser-specific output using the provided inner parsers.
    fn raw_parser<'a, 'b>(
        inner: RecursiveParser<'a, 'b, Self::RecursiveParserOutput<'a>>
    ) -> impl DefaultParser<'a, Self::Output<'a>>;

    /// Converts the result of calling [`Parser::parse`] on the parser returned by
    /// [`RecursiveParser::raw_parser`].
    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a>;
}