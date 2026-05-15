use crate::parsers::{composed::Composed, keywords::Keyword, primitives::Primitive, quoted::Quoted};

/// Keywords that should be treates in a special fashion.
/// Useful specially for cons & lists
pub const KEYWORDS: &[&str] = &[
    "lambda",
    "define",
    "if",
    "cond"
];

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // -- PRIMITIVES --
    Primitive(Primitive<'a>),

    Composed(Composed<'a>),
    Quoted(Quoted<'a>),
    Keyword(Keyword<'a>)
}