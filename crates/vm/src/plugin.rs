use std::ops::Deref;

use common::value::{GCValue, Value, ValueError, ValueExt};
use thiserror::Error;

use crate::{native::NativePlugins, vm::VM};

/// API for native functions, which are functions implemented in Rust and callable from Racket code.
#[derive(Debug, Clone, Copy)]
pub struct NativePlugin {
    /// Namespace of the native function, used to group related functions together..
    pub namespace: &'static str,
    /// Name of the native function, used to identify the function when called from Racket code.
    pub name: &'static str,
    /// Arity of the native function, used to check if the correct number of arguments is passed when called from Racket code.
    ///
    /// The returned tuple contains the number of arguments (minus variadic if present)
    /// and a boolean indicating if the function is variadic.
    pub arity: (usize, bool),
    /// Calls the native function with the given arguments and returns a value or an error.
    pub call: fn(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError>,
}

impl NativePlugin {
    /// Namespace of the native function, used to group related functions together..
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }
    /// Name of the native function, used to identify the function when called from Racket code.
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Arity of the native function, used to check if the correct number of arguments is passed when called from Racket code.
    ///
    /// The returned tuple contains the number of arguments (minus variadic if present)
    /// and a boolean indicating if the function is variadic.
    pub fn arity(&self) -> (usize, bool) {
        self.arity
    }
    /// Calls the native function with the given arguments and returns a value or an error.
    fn call(&self, vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
        (self.call)(vm, args)
    }

    /// Tries to call the native function, checking if the correct number of arguments is passed based on the arity and variadic properties of the function.
    pub fn try_call(&self, vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
        let (arity, variadic) = self.arity();
        if args.len() < arity || (!variadic && args.len() > arity) {
            return Err(NativeError::InvalidArity {
                expected: arity,
                variadic,
                got: args.len(),
            });
        }

        self.call(vm, args)
    }
}

pub trait NativePluginCollection {
    fn register(self, registry: &mut NativePlugins);
}

/// Type returned by native functions, can be either a GCValue or a Value. Function
/// returns must avoid creating GC values directly if created newly on return.
pub enum MaybeGcValue {
    Gc(GCValue),
    Value(Value),
}

impl From<Value> for MaybeGcValue {
    fn from(value: Value) -> Self {
        MaybeGcValue::Value(value)
    }
}

impl From<GCValue> for MaybeGcValue {
    fn from(value: GCValue) -> Self {
        MaybeGcValue::Gc(value)
    }
}

impl Deref for MaybeGcValue {
    type Target = Value;
    
    fn deref(&self) -> &Self::Target {
        match self {
            MaybeGcValue::Gc(gc_value) => gc_value.as_ref(),
            MaybeGcValue::Value(value) => value,
        }   
    }
}

impl ValueExt for MaybeGcValue {
    fn into_gc_value(self) -> GCValue {
        match self {
            MaybeGcValue::Gc(gc_value) => gc_value,
            MaybeGcValue::Value(value) => GCValue::new(value),
        }
    }

    fn into_value(self) -> Value {
        match self {
            MaybeGcValue::Gc(gc_value) => gc_value.into_value(),
            MaybeGcValue::Value(value) => value,
        }
    }
}

#[derive(Debug, Error)]
pub enum NativeError {
    #[error("Invalid number of arguments passed to native function. Expected {expected}{} but got {got}.", if *variadic { " or more" } else { "" })]
    InvalidArity {
        expected: usize,
        variadic: bool,
        got: usize,
    },
    #[error(transparent)]
    ValueError(#[from] ValueError),
    #[error("Invalid type of argument, got {got}, expected {expected}")]
    InvalidType {
        expected: &'static str,
        got: &'static str,
    },
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    VmError(#[from] Box<crate::error::Error>),
}
