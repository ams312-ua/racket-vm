use common::bytecode::BytecodeInstruction;

/// Control flow marker that indicates what to do after a subprogram call returns
pub enum ControlFrame {
    /// After an if condition is evaluated, select the next frame based on the result
    IfSelect {
        then_branch: Vec<BytecodeInstruction>,
        else_branch: Vec<BytecodeInstruction>,
    },
    /// After a cond condition is evaluated, select the next frame based on the result
    CondSelect {
        next_index: usize,
        conditions: Vec<Vec<BytecodeInstruction>>,
        branches: Vec<Vec<BytecodeInstruction>>,
        else_branch: Option<Vec<BytecodeInstruction>>,
    },
    /// After a function call returns, push the result onto the result map, that will be used to resolve it
    /// on a native call, to get the result.
    CallResult {
        marker: usize,
    }
}