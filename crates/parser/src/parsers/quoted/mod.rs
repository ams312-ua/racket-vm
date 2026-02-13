pub mod any_quoted;
pub mod list;
pub mod cons;

use either::Either;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Quoted<'a> {
    List(Vec<Either<Token<'a>, Quoted<'a>>>),
    Cons {
        left: Box<Either<Token<'a>, Quoted<'a>>>,
        right: Box<Either<Token<'a>, Quoted<'a>>>
    }
}
