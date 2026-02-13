use either::Either;

use crate::token::Token;

pub mod args;
pub mod body;
pub mod cons;
pub mod function;
pub mod lambda;
pub mod any_composed;
pub mod call;

/// Representation of a function's arguments.
///
/// `FnArgs` stores the list of named parameters and an optional variadic
/// identifier. Note that the `ArgsParser` which produces this type parses the
/// argument list itself and DOES NOT consume surrounding delimiters such as
/// parentheses — this lets the same parser be reused by lambdas and other
/// higher-level forms which handle delimiters.
#[derive(Clone, Debug)]
pub struct FnArgs<'a> {
    pub named: Vec<&'a str>,
    pub variadic: Option<&'a str>,
}

/// Representation of a function's body.
/// /// A `FnBody` can either be a single expression (token) or a sequence of
/// more.
/// 
/// Examples:
/// ```text
/// 42
/// (begin (define x 10) (define y 20) (+ x y))
/// ```
#[derive(Clone, Debug)]
pub struct FnBody<'a> {
    pub content: Vec<Either<Token<'a>, Composed<'a>>>,
}

/// Representation of a tree-like composed expression.
/// A `Tree` has a root token (usually an operator or function name) and a list
/// of leaves, which can be either tokens or other composed expressions.
/// 
/// Examples:
/// ```text
/// (+ 1 2)
/// (* (+ 1 2) 3)
/// ```
#[derive(Clone, Debug)]
pub struct Tree<'a> {
    pub root: &'a str,
    pub leaves: Vec<Either<Token<'a>, Composed<'a>>>,
}

#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub name: Option<&'a str>,
    pub args: FnArgs<'a>,
    pub body: Box<FnBody<'a>>,
}

#[derive(Clone, Debug)]
pub enum Composed<'a> {
    /// Lambda / function definition.
    Function(Function<'a>),
    Tree(Tree<'a>),
    Define {
        name: &'a str,
        value: Box<Either<Token<'a>, Composed<'a>>>,
    },
    Cons {
        head: Box<Either<Token<'a>, Composed<'a>>>,
        tail: Box<Either<Token<'a>, Composed<'a>>>,
    }
}