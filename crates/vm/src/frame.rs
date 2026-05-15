use std::collections::HashMap;

use common::{
    bytecode::BytecodeInstruction,
    value::{GCValue, Value},
};

use crate::{stack::Stack, storage::ValueStorage};

/// A frame represents a single execution context in the virtual machine.
///
/// Contains information about the current function being executed, its local variables and more.
#[derive(Debug, Clone)]
pub struct Frame {
    pub locals: ValueStorage,
    bytecode: Vec<BytecodeInstruction>,
    is_root: bool,
}

impl Frame {
    pub fn new(mut bytecode: Vec<BytecodeInstruction>) -> Self {
        bytecode.reverse(); // Reverse bytecode to allow for efficient popping from the end
        Self {
            locals: ValueStorage::new(),
            bytecode,
            is_root: true,
        }
    }

    /// Binds a new variable in the current frame.
    pub fn bind(&mut self, name: Box<str>, value: GCValue) {
        self.locals.insert(name, value);
    }

    pub fn make_child(&self, new_bytecode: Vec<BytecodeInstruction>) -> Self {
        let mut child = Self::new(new_bytecode);
        child.locals = self.locals.clone();
        child.is_root = false;
        child
    }

    pub fn next_instruction(&mut self) -> Option<BytecodeInstruction> {
        self.bytecode.pop()
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }
}
