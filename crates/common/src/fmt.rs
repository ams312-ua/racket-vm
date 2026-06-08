use std::fmt::{Display, Formatter, Result};

use crate::value::Value;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use super::value::Value::*;

        match self {
            String(s) => write!(f, "\"{}\"", s),
            Character(c) => write!(f, "#\\{}", c),
            Bytes(b) => write!(f, "#\"{}\"", b.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join("")),
            Integer(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            Double(d) => write!(f, "{}", d),
            Hex(h) => write!(f, "{}", h),
            Binary(b) => write!(f, "{}", b),
            Octal(o) => write!(f, "{}", o),
            Regex(r) => write!(f, "#rx\"{}\"", r),
            Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Identifier(i) => write!(f, "#{}", i),
            Pair { car, cdr, is_list } => {
                if *is_list {
                    write!(f, "'({}", car.as_ref())?;
                    let mut current_cdr = cdr.as_ref();
                    while let Pair { car, cdr, is_list } = current_cdr {
                        write!(f, " {}", car.as_ref())?;
                        if !*is_list {
                            write!(f, " . {}", cdr.as_ref())?;
                            break;
                        }
                        current_cdr = cdr.as_ref();
                    }
                    write!(f, ")")
                } else {
                    write!(f, "'({} . {})", car.as_ref(), cdr.as_ref())
                }
            }
            Function(name) => write!(f, "#<procedure:{}>", name),
            Void => write!(f, "#<void>"),
            Null => write!(f, "'()"),
        }
    }
}
