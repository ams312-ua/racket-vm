use crate::parsers::{composed::Composed, primitives::Primitive, quoted::Quoted};

pub const KEYWORDS: &[&str] = &[
    "lambda",
    "define"
];

#[derive(Debug, Clone)]
pub enum Token<'a> {
    // -- PRIMITIVES --
    Primitive(Primitive<'a>),

    Composed(Composed<'a>),
    Quoted(Quoted<'a>)
}