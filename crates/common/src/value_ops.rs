use crate::value::{Value, ValueError};

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f32 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double(*a as f64 + b)),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a + *b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok(Value::Double(*a as f64 + b)),
            (Value::Double(a), Value::Float(b)) => Ok(Value::Double(a + *b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "addition",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f32 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double(*a as f64 - b)),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a - *b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok(Value::Double(*a as f64 - b)),
            (Value::Double(a), Value::Float(b)) => Ok(Value::Double(a - *b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "subtraction",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f32 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double(*a as f64 * b)),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a * *b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok(Value::Double(*a as f64 * b)),
            (Value::Double(a), Value::Float(b)) => Ok(Value::Double(a * *b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "multiplication",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn div(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a / b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f32 / b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a / *b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double(*a as f64 / b)),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a / *b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok(Value::Double(*a as f64 / b)),
            (Value::Double(a), Value::Float(b)) => Ok(Value::Double(a / *b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "division",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn expt(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.pow(*b as u32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a.powf(*b))),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f32).powf(*b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(*b as f32))),
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double((*a as f64).powf(*b))),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a.powf(*b as f64))),
            (Value::Float(a), Value::Double(b)) => Ok(Value::Double((*a as f64).powf(*b))),
            (Value::Double(a), Value::Float(b)) => Ok(Value::Double(a.powf(*b as f64))),
            _ => Err(ValueError::InvalidOperation {
                operation: "exponentiation",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn gt(&self, other: &Value) -> Result<bool, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            (Value::Double(a), Value::Double(b)) => Ok(a > b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f32) > *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a > (*b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok((*a as f64) > *b),
            (Value::Double(a), Value::Integer(b)) => Ok(*a > (*b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok((*a as f64) > *b),
            (Value::Double(a), Value::Float(b)) => Ok(*a > (*b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "greater than",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
       }
    }

    pub fn lt(&self, other: &Value) -> Result<bool, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            (Value::Double(a), Value::Double(b)) => Ok(a < b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f32) < *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a < (*b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok((*a as f64) < *b),
            (Value::Double(a), Value::Integer(b)) => Ok(*a < (*b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok((*a as f64) < *b),
            (Value::Double(a), Value::Float(b)) => Ok(*a < (*b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "less than",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
        }
    }

    pub fn ge(&self, other: &Value) -> Result<bool, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a >= b),
            (Value::Float(a), Value::Float(b)) => Ok(a >= b),
            (Value::Double(a), Value::Double(b)) => Ok(a >= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f32) >= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a >= (*b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok((*a as f64) >= *b),
            (Value::Double(a), Value::Integer(b)) => Ok(*a >= (*b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok((*a as f64) >= *b),
            (Value::Double(a), Value::Float(b)) => Ok(*a >= (*b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "greater than or equal to",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
        }
    }

    pub fn le(&self, other: &Value) -> Result<bool, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a <= b),
            (Value::Float(a), Value::Float(b)) => Ok(a <= b),
            (Value::Double(a), Value::Double(b)) => Ok(a <= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f32) <= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a <= (*b as f32)),
            (Value::Integer(a), Value::Double(b)) => Ok((*a as f64) <= *b),
            (Value::Double(a), Value::Integer(b)) => Ok(*a <= (*b as f64)),
            (Value::Float(a), Value::Double(b)) => Ok((*a as f64) <= *b),
            (Value::Double(a), Value::Float(b)) => Ok(*a <= (*b as f64)),
            _ => Err(ValueError::InvalidOperation {
                operation: "less than or equal to",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
        }
    }

    pub fn eq(&self, other: &Value) -> Result<bool, ValueError> {
        let res = match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Character(a), Value::Character(b)) => a == b,
            (Value::Bytes(a), Value::Bytes(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Double(a), Value::Double(b)) => a == b,
            (Value::Hex(a), Value::Hex(b)) => a == b,
            (Value::Binary(a), Value::Binary(b)) => a == b,
            (Value::Octal(a), Value::Octal(b)) => a == b,
            (Value::Regex(a), Value::Regex(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            _ => return Err(ValueError::InvalidOperation {
                operation: "equality",
                left: self.data_type_name(),
                right: other.data_type_name(),
            })
        };

        Ok(res)
    }
}