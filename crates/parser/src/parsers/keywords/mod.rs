use either::Either;

use crate::{parsers::composed::Composed, token::Token};

pub mod define;

#[derive(Debug, Clone)]
pub enum Keyword<'a> {
    Define {
        name: &'a str,
        value: Box<Token<'a>>,
    }
}