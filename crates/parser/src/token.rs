use crate::parsers::{composed::Composed, primitives::Primitive, quoted::Quoted};

pub const KEYWORDS: &[&str] = &[
    "lambda",
];

#[derive(Debug, Clone)]
pub enum Token<'a> {
    // -- PRIMITIVES --
    Primitive(Primitive<'a>),
    Pair(Box<Token<'a>>, Box<Token<'a>>),

    Composed(Composed<'a>),
    Quoted(Quoted<'a>)
}