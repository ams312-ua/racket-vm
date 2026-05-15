pub mod any_quoted;
pub mod list;
pub mod cons;

use either::Either;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Quoted<'a> {
    List(Vec<Token<'a>>),
    Cons {
        left: Box<Token<'a>>,
        right: Box<Token<'a>>
    }
}
