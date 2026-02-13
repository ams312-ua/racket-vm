use crate::{parsers::{DefaultParser, RParser, primitives::Primitive}, token::Token};
use chumsky::prelude::*;

/// Parser for Racket identifiers.
///
/// Recognizes Racket symbol/identifier names.
/// 
/// Identifiers can start with:
/// - Letters (a-z, A-Z)
/// - Underscores (_)
/// - Special characters (+, -, *, /, <, >, =, !, ?, @, #, $, %, ^, &, ~, .)
///
/// Subsequent characters can be any of the above plus digits (0-9).
///
/// Examples:
/// ```text
/// foo
/// my-function
/// +
/// foo?
/// bar!
/// my_var
/// ```
#[derive(Clone, Copy)]
pub struct IdentifierParser;

impl RParser for IdentifierParser {
    type Output<'a> = Primitive<'a>;

    fn raw_parser<'a>() -> impl DefaultParser<'a, Self::Output<'a>> {
        text::ident()
            .ignored()
            .or(one_of("_+-*/<>=!?@#$%^&~.").ignored())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(Primitive::Ident)
    }

    fn to_token<'a>(src: Self::Output<'a>) -> Token<'a> {
        Token::Primitive(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ident(input: &str) -> Result<Primitive, String> {
        match IdentifierParser::raw_parser().parse(input).into_result() {
            Ok(primitive) => Ok(primitive),
            Err(e) => Err(format!("Parse error: {:?}", e)),
        }
    }

    #[test]
    fn test_simple_identifiers() {
        assert!(matches!(parse_ident("foo"), Ok(Primitive::Ident("foo"))));
        assert!(matches!(parse_ident("bar"), Ok(Primitive::Ident("bar"))));
        assert!(matches!(parse_ident("x"), Ok(Primitive::Ident("x"))));
        assert!(matches!(parse_ident("X"), Ok(Primitive::Ident("X"))));
        assert!(matches!(parse_ident("Z"), Ok(Primitive::Ident("Z"))));
    }

    #[test]
    fn test_uppercase_identifiers() {
        assert!(matches!(parse_ident("FOO"), Ok(Primitive::Ident("FOO"))));
        assert!(matches!(parse_ident("BAR"), Ok(Primitive::Ident("BAR"))));
        assert!(matches!(parse_ident("HELLO_WORLD"), Ok(Primitive::Ident("HELLO_WORLD"))));
    }

    #[test]
    fn test_lowercase_identifiers() {
        assert!(matches!(parse_ident("hello"), Ok(Primitive::Ident("hello"))));
        assert!(matches!(parse_ident("world"), Ok(Primitive::Ident("world"))));
        assert!(matches!(parse_ident("abcdefghijklmnopqrstuvwxyz"), Ok(Primitive::Ident(_))));
    }

    #[test]
    fn test_identifiers_with_underscores() {
        assert!(matches!(parse_ident("_"), Ok(Primitive::Ident("_"))));
        assert!(matches!(parse_ident("_foo"), Ok(Primitive::Ident("_foo"))));
        assert!(matches!(parse_ident("foo_bar"), Ok(Primitive::Ident("foo_bar"))));
        assert!(matches!(parse_ident("_bar_baz_"), Ok(Primitive::Ident("_bar_baz_"))));
        assert!(matches!(parse_ident("___"), Ok(Primitive::Ident("___"))));
    }

    #[test]
    fn test_identifiers_with_hyphens() {
        assert!(matches!(parse_ident("-"), Ok(Primitive::Ident("-"))));
        assert!(matches!(parse_ident("my-var"), Ok(Primitive::Ident("my-var"))));
        assert!(matches!(parse_ident("foo-bar-baz"), Ok(Primitive::Ident("foo-bar-baz"))));
        assert!(matches!(parse_ident("-foo"), Ok(Primitive::Ident("-foo"))));
        assert!(matches!(parse_ident("foo-"), Ok(Primitive::Ident("foo-"))));
    }

    #[test]
    fn test_identifiers_with_digits() {
        assert!(matches!(parse_ident("foo1"), Ok(Primitive::Ident("foo1"))));
        assert!(matches!(parse_ident("var2name"), Ok(Primitive::Ident("var2name"))));
        assert!(matches!(parse_ident("test123"), Ok(Primitive::Ident("test123"))));
        assert!(matches!(parse_ident("a0b1c2"), Ok(Primitive::Ident("a0b1c2"))));
        assert!(matches!(parse_ident("x9"), Ok(Primitive::Ident("x9"))));
    }

    #[test]
    fn test_special_single_char_identifiers() {
        assert!(matches!(parse_ident("+"), Ok(Primitive::Ident("+"))));
        assert!(matches!(parse_ident("*"), Ok(Primitive::Ident("*"))));
        assert!(matches!(parse_ident("/"), Ok(Primitive::Ident("/"))));
        assert!(matches!(parse_ident("<"), Ok(Primitive::Ident("<"))));
        assert!(matches!(parse_ident(">"), Ok(Primitive::Ident(">"))));
        assert!(matches!(parse_ident("="), Ok(Primitive::Ident("="))));
        assert!(matches!(parse_ident("!"), Ok(Primitive::Ident("!"))));
        assert!(matches!(parse_ident("?"), Ok(Primitive::Ident("?"))));
        assert!(matches!(parse_ident("@"), Ok(Primitive::Ident("@"))));
        assert!(matches!(parse_ident("#"), Ok(Primitive::Ident("#"))));
        assert!(matches!(parse_ident("$"), Ok(Primitive::Ident("$"))));
        assert!(matches!(parse_ident("%"), Ok(Primitive::Ident("%"))));
        assert!(matches!(parse_ident("^"), Ok(Primitive::Ident("^"))));
        assert!(matches!(parse_ident("&"), Ok(Primitive::Ident("&"))));
        assert!(matches!(parse_ident("~"), Ok(Primitive::Ident("~"))));
        assert!(matches!(parse_ident("."), Ok(Primitive::Ident("."))));
    }

    #[test]
    fn test_identifiers_with_question_marks() {
        assert!(matches!(parse_ident("foo?"), Ok(Primitive::Ident("foo?"))));
        assert!(matches!(parse_ident("is-empty?"), Ok(Primitive::Ident("is-empty?"))));
        assert!(matches!(parse_ident("number?"), Ok(Primitive::Ident("number?"))));
        assert!(matches!(parse_ident("?foo"), Ok(Primitive::Ident("?foo"))));
    }

    #[test]
    fn test_identifiers_with_exclamation_marks() {
        assert!(matches!(parse_ident("foo!"), Ok(Primitive::Ident("foo!"))));
        assert!(matches!(parse_ident("set!"), Ok(Primitive::Ident("set!"))));
        assert!(matches!(parse_ident("!bar"), Ok(Primitive::Ident("!bar"))));
    }

    #[test]
    fn test_identifiers_with_dots() {
        assert!(matches!(parse_ident("."), Ok(Primitive::Ident("."))));
        assert!(matches!(parse_ident("..."), Ok(Primitive::Ident("..."))));
        assert!(matches!(parse_ident(".foo"), Ok(Primitive::Ident(".foo"))));
        assert!(matches!(parse_ident("foo.bar"), Ok(Primitive::Ident("foo.bar"))));
    }

    #[test]
    fn test_identifiers_with_arithmetic_operators() {
        assert!(matches!(parse_ident("+"), Ok(Primitive::Ident("+"))));
        assert!(matches!(parse_ident("-"), Ok(Primitive::Ident("-"))));
        assert!(matches!(parse_ident("*"), Ok(Primitive::Ident("*"))));
        assert!(matches!(parse_ident("/"), Ok(Primitive::Ident("/"))));
        assert!(matches!(parse_ident("+foo+"), Ok(Primitive::Ident("+foo+"))));
        assert!(matches!(parse_ident("*square*"), Ok(Primitive::Ident("*square*"))));
    }

    #[test]
    fn test_identifiers_with_comparison_operators() {
        assert!(matches!(parse_ident("<"), Ok(Primitive::Ident("<"))));
        assert!(matches!(parse_ident(">"), Ok(Primitive::Ident(">"))));
        assert!(matches!(parse_ident("="), Ok(Primitive::Ident("="))));
        assert!(matches!(parse_ident("<="), Ok(Primitive::Ident("<="))));
        assert!(matches!(parse_ident(">="), Ok(Primitive::Ident(">="))));
        assert!(matches!(parse_ident("<>"), Ok(Primitive::Ident("<>"))));
        assert!(matches!(parse_ident("eq?"), Ok(Primitive::Ident("eq?"))));
    }

    #[test]
    fn test_complex_identifiers() {
        assert!(matches!(parse_ident("map-filter-reduce"), Ok(Primitive::Ident("map-filter-reduce"))));
        assert!(matches!(parse_ident("list->vector"), Ok(Primitive::Ident("list->vector"))));
        assert!(matches!(parse_ident("vector->list"), Ok(Primitive::Ident("vector->list"))));
        assert!(matches!(parse_ident("string->number"), Ok(Primitive::Ident("string->number"))));
        assert!(matches!(parse_ident("number->string"), Ok(Primitive::Ident("number->string"))));
    }

    #[test]
    fn test_common_racket_keywords_and_functions() {
        assert!(matches!(parse_ident("define"), Ok(Primitive::Ident("define"))));
        assert!(matches!(parse_ident("lambda"), Ok(Primitive::Ident("lambda"))));
        assert!(matches!(parse_ident("let"), Ok(Primitive::Ident("let"))));
        assert!(matches!(parse_ident("if"), Ok(Primitive::Ident("if"))));
        assert!(matches!(parse_ident("cond"), Ok(Primitive::Ident("cond"))));
        assert!(matches!(parse_ident("quote"), Ok(Primitive::Ident("quote"))));
        assert!(matches!(parse_ident("cons"), Ok(Primitive::Ident("cons"))));
        assert!(matches!(parse_ident("car"), Ok(Primitive::Ident("car"))));
        assert!(matches!(parse_ident("cdr"), Ok(Primitive::Ident("cdr"))));
        assert!(matches!(parse_ident("list"), Ok(Primitive::Ident("list"))));
        assert!(matches!(parse_ident("append"), Ok(Primitive::Ident("append"))));
    }

    #[test]
    fn test_predicate_identifiers() {
        assert!(matches!(parse_ident("null?"), Ok(Primitive::Ident("null?"))));
        assert!(matches!(parse_ident("pair?"), Ok(Primitive::Ident("pair?"))));
        assert!(matches!(parse_ident("list?"), Ok(Primitive::Ident("list?"))));
        assert!(matches!(parse_ident("string?"), Ok(Primitive::Ident("string?"))));
        assert!(matches!(parse_ident("symbol?"), Ok(Primitive::Ident("symbol?"))));
        assert!(matches!(parse_ident("procedure?"), Ok(Primitive::Ident("procedure?"))));
        assert!(matches!(parse_ident("even?"), Ok(Primitive::Ident("even?"))));
        assert!(matches!(parse_ident("odd?"), Ok(Primitive::Ident("odd?"))));
    }

    #[test]
    fn test_mutation_identifiers() {
        assert!(matches!(parse_ident("set!"), Ok(Primitive::Ident("set!"))));
        assert!(matches!(parse_ident("set-car!"), Ok(Primitive::Ident("set-car!"))));
        assert!(matches!(parse_ident("set-cdr!"), Ok(Primitive::Ident("set-cdr!"))));
        assert!(matches!(parse_ident("string-set!"), Ok(Primitive::Ident("string-set!"))));
        assert!(matches!(parse_ident("vector-set!"), Ok(Primitive::Ident("vector-set!"))));
    }

    #[test]
    fn test_mixed_special_chars() {
        assert!(matches!(parse_ident("+-*"), Ok(Primitive::Ident("+-*"))));
        assert!(matches!(parse_ident("<=>"), Ok(Primitive::Ident("<=>"))));
        assert!(matches!(parse_ident("!@#$"), Ok(Primitive::Ident("!@#$"))));
        assert!(matches!(parse_ident("%^&~"), Ok(Primitive::Ident("%^&~"))));
        assert!(matches!(parse_ident("+-/<>=!?"), Ok(Primitive::Ident("+-/<>=!?"))));
    }

    #[test]
    fn test_long_identifiers() {
        let long_ident = "this_is_a_very_long_identifier_with_many_characters_and_underscores_0123456789";
        assert!(matches!(parse_ident(long_ident), Ok(Primitive::Ident(_))));
    }

    #[test]
    fn test_underscore_variations() {
        assert!(matches!(parse_ident("_"), Ok(Primitive::Ident("_"))));
        assert!(matches!(parse_ident("__"), Ok(Primitive::Ident("__"))));
        assert!(matches!(parse_ident("___________"), Ok(Primitive::Ident("___________"))));
        assert!(matches!(parse_ident("_a_b_c_"), Ok(Primitive::Ident("_a_b_c_"))));
    }

    #[test]
    fn test_mixed_case_identifiers() {
        assert!(matches!(parse_ident("camelCase"), Ok(Primitive::Ident("camelCase"))));
        assert!(matches!(parse_ident("PascalCase"), Ok(Primitive::Ident("PascalCase"))));
        assert!(matches!(parse_ident("MiXeD_CaSe_123"), Ok(Primitive::Ident("MiXeD_CaSe_123"))));
    }

    #[test]
    fn test_arrow_like_identifiers() {
        assert!(matches!(parse_ident("->"), Ok(Primitive::Ident("->"))));
        assert!(matches!(parse_ident("<-"), Ok(Primitive::Ident("<-"))));
        assert!(matches!(parse_ident("=>"), Ok(Primitive::Ident("=>"))));
        assert!(matches!(parse_ident("<->"), Ok(Primitive::Ident("<->"))));
        assert!(matches!(parse_ident("->>"), Ok(Primitive::Ident("->>" ))));
    }

    #[test]
    fn test_single_letter_all_cases() {
        for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            let s = c.to_string();
            assert!(parse_ident(&s).is_ok(), "Failed for: {}", s);
        }
    }

    #[test]
    fn test_all_special_char_combinations() {
        let special_chars = "+-*/<>=!?@#$%^&~.";
        for c in special_chars.chars() {
            let s = c.to_string();
            assert!(parse_ident(&s).is_ok(), "Failed for: {}", s);
        }
    }

    #[test]
    fn test_all_digits_prefixed_with_letter() {
        for digit in 0..=9 {
            let s = format!("x{}", digit);
            assert!(matches!(parse_ident(&s), Ok(Primitive::Ident(_))));
        }
    }
}