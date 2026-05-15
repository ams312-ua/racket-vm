use either::Either;

use crate::{parsers::composed::Composed, token::Token};

pub mod cond;
pub mod define;
pub mod r#if;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword<'a> {
    Define {
        name: &'a str,
        value: Box<Token<'a>>,
    },

    If {
        condition: Box<Token<'a>>,
        then_branch: Box<Token<'a>>,
        else_branch: Box<Token<'a>>,
    },

    Cond {
        branches: Vec<(Token<'a>, Token<'a>)>,
        else_branch: Option<Box<Token<'a>>>,
    }
}