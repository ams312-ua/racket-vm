use common::bytecode::BytecodeInstruction;

/// Buffer where all bytecode instructions are stored before being emitted.
pub struct BytecodeBuffer {
    instructions: Vec<BytecodeInstruction>,
}

/// Buffers used by the compiler in order to store data, reorganize it, and modify it.
pub struct Buffers<'a> {
    /// Buffer for bytecode that should be prepended to the current instruction stream.
    prepend_buffer: Vec<BytecodeInstruction>,
    /// Normal bytecode instruction flow buffer.
    buffer: Vec<BytecodeInstruction>,
    /// Buffer for bytecode that should be appended to the current instruction stream.
    ///
    /// Disabled by default, but inherits parent buffer enabling when reborrowed.
    tail_buffer: Option<Vec<BytecodeInstruction>>,
    /// Parent, where all instructions are emitted to after this instance gets dropped.
    parent: &'a mut BytecodeBuffer,
}

impl BytecodeBuffer {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            instructions: Vec::with_capacity(capacity),
        }
    }

    /// Creates the first buffer, which can be used to begin compilation.
    pub fn begin(&mut self) -> Buffers {
        Buffers {
            prepend_buffer: Vec::new(),
            buffer: Vec::new(),
            tail_buffer: None,
            parent: self,
        }
    }

    pub fn into_instructions(self) -> Vec<BytecodeInstruction> {
        self.instructions
    }
}

impl<'a> Buffers<'a> {
    /// Creates a new buffer with the same parent as the current buffer.
    ///
    /// This is useful for compiling nested functions, where we want to compile the inner function first and then emit it as a value in the outer function.
    pub fn reborrow(&mut self) -> Buffers {
        Buffers {
            prepend_buffer: Vec::new(),
            buffer: Vec::new(),
            tail_buffer: if self.tail_buffer.is_some() {
                Some(Vec::new())
            } else {
                None
            },
            parent: self.parent,
        }
    }

    pub fn is_tail_buffer_enabled(&self) -> bool {
        self.tail_buffer.is_some()
    }

    pub fn enable_tail_buffer(&mut self) -> &mut Self {
        self.tail_buffer = Some(Vec::new());
        self
    }

    pub fn disable_tail_buffer(&mut self) -> &mut Self {
        self.flush_tail_buffer();
        self.tail_buffer = None;
        self
    }

    pub fn flush_tail_buffer(&mut self) -> &mut Self {
        if let Some(tail_buffer) = self.tail_buffer.take() {
            self.buffer.extend(tail_buffer);
        }
        self
    }

    /// Adds an instruction to the prepend buffer, which will be emitted before all other instructions in the current buffer when this instance gets dropped.
    pub fn prepend(&mut self, instruction: BytecodeInstruction) {
        self.prepend_buffer.push(instruction);
    }

    /// Adds an instruction to the normal buffer, which will be emitted after all instructions in the prepend buffer when this instance gets dropped.
    pub fn emit(&mut self, instruction: BytecodeInstruction) {
        self.buffer.push(instruction);
    }

    /// Adds an instruction to the tail buffer, which will be emitted after all instructions in the normal buffer when this instance gets dropped.
    ///
    /// This only works if the tail buffer is enabled.
    pub fn defer(&mut self, instruction: BytecodeInstruction) {
        self.tail_buffer.as_mut().map(|buf| buf.push(instruction));
    }
}

impl<'a> Drop for Buffers<'a> {
    fn drop(&mut self) {
        // When this buffer goes out of scope, we need to emit all the instructions in the prepend buffer, then all the instructions in the normal buffer, into the parent buffer.
        self.parent
            .instructions
            .extend(self.prepend_buffer.drain(..));
        self.parent.instructions.extend(self.buffer.drain(..));
        if let Some(tail_buffer) = self.tail_buffer.take() {
            self.parent.instructions.extend(tail_buffer);
        }
    }
}
