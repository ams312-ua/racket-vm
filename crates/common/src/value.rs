use std::fmt::Debug;

use dumpster::Trace;
use dumpster::unsync::Gc;
use parser::parsers::primitives::Primitive;
use thiserror::Error;

pub type GCValue = Gc<Value>;

pub mod garbage_collection {
    use dumpster::unsync::collect;
}

pub trait GCValueExt {
    fn into_value(&self) -> Value;
    fn iter_list(&self) -> Result<impl Iterator<Item = GCValue>, ValueError>;
}

impl GCValueExt for GCValue {
    fn into_value(&self) -> Value {
        self.as_ref().clone()
    }

    fn iter_list(&self) -> Result<impl Iterator<Item = GCValue>, ValueError> {
        if !self.as_ref().is_list() {
            return Err(ValueError::Conversion {
                from: self.as_ref().data_type_name(),
                to: "list",
            });
        }

        Ok(crate::list_iter::ListIter::new(self.clone()))
    }
}

pub trait ValueExt {
    fn into_gc_value(self) -> GCValue;
    fn into_value(self) -> Value;
}

impl ValueExt for Value {
    fn into_gc_value(self) -> GCValue {
        GCValue::new(self)
    }

    fn into_value(self) -> Value {
        self
    }
}

impl ValueExt for GCValue {
    fn into_gc_value(self) -> GCValue {
        self
    }

    fn into_value(self) -> Value {
        <GCValue as GCValueExt>::into_value(&self)
    }
}

/// All primitive values that can be used by the VM.
#[derive(Clone, PartialEq, Trace)]
pub enum Value {
    String(Box<str>),
    Character(Box<str>),
    Bytes(Box<[u8]>),
    Integer(i64),
    Float(f32),
    Double(f64),
    Hex(usize),
    Binary(usize),
    Octal(usize),
    Regex(Box<str>),
    Boolean(bool),
    Identifier(Box<str>),
    /// Composed type for lists and pairs.
    /// Lists are implemented as pairs where the cdr is either another pair or null (empty list).
    Pair {
        car: GCValue,
        cdr: GCValue,
        is_list: bool, // True if this pair is part of a list (i.e., cdr is either another pair or null)
    },
    Function(Box<str>),
    Void,
    Null, // Also known as empty list -> '()
}

impl<'a> From<Primitive<'a>> for Value {
    fn from(primitive: Primitive<'a>) -> Self {
        match primitive {
            Primitive::String(s) => Value::String(s.into()),
            Primitive::Character(c) => Value::Character(c.into()),
            Primitive::Bytes(b) => Value::Bytes(b.into()),
            Primitive::Integer(i) => Value::Integer(i),
            Primitive::Float(f) => Value::Float(f),
            Primitive::Double(d) => Value::Double(d),
            Primitive::Hex(h) => Value::Hex(usize::from_str_radix(h, 16).unwrap_or(0)),
            Primitive::Binary(b) => Value::Binary(usize::from_str_radix(b, 2).unwrap_or(0)),
            Primitive::Octal(o) => Value::Octal(usize::from_str_radix(o, 8).unwrap_or(0)),
            Primitive::Regex(r) => Value::Regex(r.into()),
            Primitive::Boolean(b) => Value::Boolean(b),
            Primitive::Ident(i) => Value::Identifier(i.into()),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "String({s})"),
            Value::Character(c) => write!(f, "Character({c})"),
            Value::Bytes(b) => write!(f, "Bytes({:?})", b),
            Value::Integer(i) => write!(f, "Integer({i})"),
            Value::Float(fl) => write!(f, "Float({fl})"),
            Value::Double(d) => write!(f, "Double({d})"),
            Value::Hex(h) => write!(f, "Hex({h:#x})"),
            Value::Binary(b) => write!(f, "Binary({b:#b})"),
            Value::Octal(o) => write!(f, "Octal({o:#o})"),
            Value::Regex(r) => write!(f, "Regex({r})"),
            Value::Boolean(b) => write!(f, "Boolean({b})"),
            Value::Identifier(i) => write!(f, "Identifier({i})"),
            Value::Pair { car, cdr, is_list } => {
                let car = car.as_ref();
                let cdr = cdr.as_ref();
                f.debug_struct("Pair")
                    .field("car", car)
                    .field("cdr", cdr)
                    .field("is_list", is_list)
                    .finish()
            }
            Value::Function(name) => write!(f, "Function({name})"),
            Value::Void => write!(f, "Void"),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl Value {
    pub fn data_type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Character(_) => "character",
            Value::Bytes(_) => "bytes",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Double(_) => "double",
            Value::Hex(_) => "hexadecimal",
            Value::Binary(_) => "binary",
            Value::Octal(_) => "octal",
            Value::Regex(_) => "regex",
            Value::Boolean(_) => "boolean",
            Value::Identifier(_) => "identifier",
            Value::Pair { is_list, .. } => {
                if *is_list {
                    "list"
                } else {
                    "pair"
                }
            }
            Value::Function(_) => "function",
            Value::Void => "void",
            Value::Null => "null",
        }
    }

    pub fn as_string(&self) -> Result<&str, ValueError> {
        if let Value::String(s) = self {
            Ok(s)
        } else {
            Err(ValueError::Conversion {
                from: self.data_type_name(),
                to: "string",
            })
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    /// Creates a new list from the specified values
    pub fn list(values: Vec<impl ValueExt>) -> Self {
        // We build the list in reverse order, starting from the empty list (Null)
        let mut current = Value::Null.into_gc_value();

        for value in values.into_iter().rev() {
            current = Value::pair(value, current).into_gc_value();
        }

        current.into_value()
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Value::Pair { is_list: true, .. })
    }

    /// Constructs a pair from the given head and tail.
    ///
    /// This is the preferred way to create pairs and lists, as it automatically handles GCValue conversion and allows for more flexible types.
    pub fn pair(car: impl ValueExt, cdr: impl ValueExt) -> Self {
        let cdr = cdr.into_gc_value();
        let is_list = cdr.check_for_list(); // Check if the cdr is a proper list
        Value::Pair {
            car: car.into_gc_value(),
            cdr: cdr,
            is_list,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// goes through the cdr of pairs verifying that the structure is a proper list.
    ///
    /// Does ignore the flag is_list, as this function is used to determine if a structure is a list.
    ///
    /// Returns true if the structure is a proper list (i.e., ends with Null and has no cycles), false otherwise.
    fn check_for_list(&self) -> bool {
        let mut current = self;

        loop {
            match current {
                Value::Pair { cdr, .. } => {
                    let cdr_ref = cdr.as_ref();
                    if let Value::Null = cdr_ref {
                        return true; // Proper list ends with Null
                    }
                    current = cdr_ref; // Move to the next pair
                }
                Value::Null => return true, // Proper list can also be just Null
                _ => return false, // If we encounter a non-pair before reaching Null, it's not a proper list
            }
        }
    }

    pub fn is_cons_like(&self) -> bool {
        // Lists are formed from pairs, so both pairs and lists are considered cons-like
        matches!(self, Value::Pair { .. })
    }
}

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("Cannot convert from {from} to {to}")]
    Conversion {
        from: &'static str,
        to: &'static str,
    },

    #[error("Cannot perform {operation} between {left} and {right}")]
    InvalidOperation {
        operation: &'static str,
        left: &'static str,
        right: &'static str,
    },
}
