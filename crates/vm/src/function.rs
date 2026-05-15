use common::bytecode::BytecodeInstruction;

/// Clone of the bytecode DefineInstruction variant for easier access
#[derive(Debug)]
pub struct DefinedFunction {
    /// Name of the variable being defined
    pub name: Box<str>,
    /// Names of the function's parameters, which will be used to bind the arguments passed to the function when it is called. The order of the parameters in this list corresponds to the order of the arguments passed to the function.
    pub args: Vec<Box<str>>,
    /// Optional name of the variadic parameter, which will be used to bind any extra arguments passed to the function when it is called. If this is None, then the function does not accept variadic arguments and an error should be raised if extra arguments are passed.
    pub variadic_arg: Option<Box<str>>,
    /// Bytecode to assign to the variable being defined
    pub bytecode: Vec<BytecodeInstruction>
}