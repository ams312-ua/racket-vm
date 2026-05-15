use common::value::ValueError;
use crate::plugin::NativeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Native(#[from] NativeError),
    #[error("Undefined function: {0}")]
    UndefinedFunction(String),
    #[error("Arity mismatch: expected {expected}{}, got {got}", if *variadic {" or more"} else {""})]
    ArityMismatch { expected: usize, variadic: bool, got: usize },
    #[error(transparent)]
    ValueError(#[from] ValueError),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Stack is empty")]
    StackEmpty,
    #[error("End of bytecode")]
    EndOfBytecode,
    #[error("VM halted unexpectedly")]
    VMHalted,
}