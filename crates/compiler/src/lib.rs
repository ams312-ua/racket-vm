use common::bytecode::BytecodeInstruction;
use common::value::{GCValue, Value};
use parser::parsers::keywords::Keyword;
use parser::parsers::primitives::Primitive;
use parser::parsers::quoted::Quoted;
use parser::token::Token;
use parser::{parsers::composed::Composed, token};

use crate::buffers::{Buffers, BytecodeBuffer};
use crate::state::State;

mod buffers;
mod state;

pub struct Compiler<'a> {
    src: Vec<&'a Token<'a>>,
    anonymous_fn_counter: usize,
    state: State,
}

impl<'a> Compiler<'a> {
    pub fn new(src: Vec<&'a Token<'a>>) -> Self {
        Self {
            src,
            anonymous_fn_counter: 0,
            state: State::new(),
        }
    }

    fn next_anonymous_function_ident(&mut self) -> Box<str> {
        self.anonymous_fn_counter += 1;
        format!("<lambda:{}>", self.anonymous_fn_counter).into_boxed_str()
    }

    fn compile_token(&mut self, token: &Token<'a>, into: &mut Buffers) {
        match token {
            Token::Primitive(primitive) => {
                let instruction = BytecodeInstruction::Push(primitive.clone().into());
                into.emit(instruction)
            }

            Token::Quoted(quoted) => {
                let value = self
                    .quoted_to_value(quoted)
                    .unwrap_or_else(|| Value::Identifier("<invalid-quoted-value>".into()));
                into.emit(BytecodeInstruction::Push(value));
            }

            Token::Composed(composed) => self.compile_composed(composed, into),

            Token::Keyword(keyword) => self.compile_keyword(keyword, into),
        }
    }

    fn compile_keyword(&mut self, keyword: &Keyword<'a>, into: &mut Buffers) {
        let inst = match keyword {
            Keyword::Define { name, value } => {
                // define normally if not function / lambda, otherwise we need to compile the function body and emit a DefineFunction instruction instead, so that it can be called by name later.
                if let Token::Composed(Composed::Function(function)) = value.as_ref() {
                    let ident = function.name.unwrap_or_else(|| *name).into();

                    // First we compile the function body into a separate buffer, so that we can emit it as part of the DefineFunction instruction.
                    let mut body_buf = BytecodeBuffer::new();

                    for token in function.body.content.iter() {
                        self.compile_token(token, &mut body_buf.begin());
                    }

                    BytecodeInstruction::DefineFunction {
                        name: ident,
                        bytecode: body_buf.into_instructions(),
                        args: function
                            .args
                            .named
                            .iter()
                            .map(|arg| (*arg).into())
                            .collect(),
                        variadic_arg: function.args.variadic.map(|arg| arg.into()),
                    }
                } else {
                    BytecodeInstruction::Define {
                        name: (*name).into(),
                        value: self.token_to_value(value.as_ref()).unwrap_or_else(|| {
                            Value::Identifier("<non-constant-expression>".into())
                        }),
                    }
                }
            }

            Keyword::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Compile condition first.
                let mut cond_buf = BytecodeBuffer::new();
                self.compile_token(condition.as_ref(), &mut cond_buf.begin());

                let mut then_buf = BytecodeBuffer::new();
                self.compile_token(then_branch.as_ref(), &mut then_buf.begin());

                let mut else_buf = BytecodeBuffer::new();
                self.compile_token(else_branch.as_ref(), &mut else_buf.begin());

                BytecodeInstruction::If {
                    condition: cond_buf.into_instructions(),
                    then_branch: then_buf.into_instructions(),
                    else_branch: else_buf.into_instructions(),
                }
            }

            Keyword::Cond {
                branches,
                else_branch,
            } => {
                let mut compiled_conditions = Vec::with_capacity(branches.len());
                let mut compiled_branches = Vec::with_capacity(branches.len());
                for (condition, branch) in branches.iter()
                /*.rev()*/
                {
                    // first compile the condition, then the branch
                    let mut cond_buf = BytecodeBuffer::new();
                    self.compile_token(condition, &mut cond_buf.begin());
                    compiled_conditions.push(cond_buf.into_instructions());

                    let mut branch_buf = BytecodeBuffer::new();
                    self.compile_token(branch, &mut branch_buf.begin());
                    compiled_branches.push(branch_buf.into_instructions());
                }
                // reverse branches so that they are in the correct index for the conditions
                // to be popped from the stack
                //compiled_branches.reverse();

                BytecodeInstruction::Cond {
                    conditions: compiled_conditions,
                    branches: compiled_branches,
                    else_branch: else_branch.as_ref().map(|branch| {
                        let mut buf = BytecodeBuffer::new();
                        self.compile_token(branch.as_ref(), &mut buf.begin());
                        buf.into_instructions()
                    }),
                }
            }
        };

        into.emit(inst);
    }

    fn quoted_to_value(&mut self, quoted: &Quoted<'a>) -> Option<Value> {
        match quoted {
            Quoted::List(items) => {
                let values = items
                    .iter()
                    .map(|item| self.token_to_value(item))
                    .map(|o| o.map(GCValue::new))
                    .collect::<Option<Vec<_>>>()?;
                Some(Value::list(values))
            }

            Quoted::Cons { left, right } => {
                let left = self.token_to_value(left.as_ref())?;
                let right = self.token_to_value(right.as_ref())?;
                Some(Value::pair(GCValue::new(left), GCValue::new(right)))
            }
        }
    }

    fn token_to_value(&mut self, token: &Token<'a>) -> Option<Value> {
        match token {
            Token::Primitive(primitive) => Some(primitive.clone().into()),
            Token::Quoted(quoted) => self.quoted_to_value(quoted),

            Token::Composed(Composed::Function(function)) => Some(Value::Function(
                function
                    .name
                    .map(Into::into)
                    .unwrap_or_else(|| self.next_anonymous_function_ident()),
            )),

            Token::Keyword(Keyword::Define { name, .. }) => Some(Value::Identifier((*name).into())),

            Token::Keyword(Keyword::If { .. }) | Token::Keyword(Keyword::Cond { .. }) => None,
            _ => None,
        }
    }

    fn compile_composed(&mut self, composed: &Composed<'a>, into: &mut Buffers) {
        match composed {
            Composed::Tree(tree) => {
                // save previous state, then set it for this tree compilation
                let snapshot = self.state.snapshot();
                self.state.push_lambda_to_stack = true;
                self.state.undefine_lambda = true;

                into.enable_tail_buffer();
                // from right to left, push all the leaves onto the stack, then call the function represented by the root with argc = number of leaves
                for leaf in tree.leaves.iter().rev() {
                    self.compile_token(leaf, into);

                    // if leaves are calls themselves, they will end up disabling the
                    // tail buffer and only a single function will be undefined at
                    // the end, so if it disabled it, just re-enable it
                    if !into.is_tail_buffer_enabled() {
                        into.enable_tail_buffer();
                    }
                }

                // Now if the root is an identifier, we can emit a call instruction directly.
                // In cases the root is a more complex expression, we need to evaluate it first and then call it.
                // For that we will evaluate, and will put the name as None, instructing the VM to call the function on top of the stack.

                let argc = tree.leaves.len();

                match tree.root.as_ref() {
                    Token::Primitive(Primitive::Ident(ident)) => {
                        into.emit(BytecodeInstruction::Call {
                            function: Some((*ident).into()),
                            argc,
                        });
                    }

                    root => {
                        self.compile_token(root, into);
                        into.emit(BytecodeInstruction::Call {
                            function: None,
                            argc,
                        });
                    }
                }

                // restore previous state
                self.state.restore(snapshot);
                into.disable_tail_buffer();
            }

            Composed::Function(function) => {
                let ident = function
                    .name
                    .map(Into::into)
                    .unwrap_or_else(|| self.next_anonymous_function_ident());

                // First we compile the function body into a separate buffer, so that we can emit it as part of the DefineFunction instruction.
                let mut body_buf = BytecodeBuffer::new();

                for token in function.body.content.iter() {
                    self.compile_token(token, &mut body_buf.begin());
                }

                into.emit(BytecodeInstruction::DefineFunction {
                    name: ident.clone(),
                    args: function
                        .args
                        .named
                        .iter()
                        .map(|arg| (*arg).into())
                        .collect(),
                    variadic_arg: function.args.variadic.map(|arg| arg.into()),
                    bytecode: body_buf.into_instructions(),
                });

                if self.state.push_lambda_to_stack {
                    into.emit(BytecodeInstruction::Push(Value::Function(ident.clone())));
                }

                if self.state.undefine_lambda {
                    // Defer function deinit. tail buffer is only enabled when function calls have lambdas
                    // so this call will be noop when calling from a define.
                    into.defer(BytecodeInstruction::Undefine { name: ident });
                }
            }
        }
    }

    fn compile_next(&mut self, into: &mut Buffers) -> Option<()> {
        if self.src.is_empty() {
            return None;
        }

        let token = self.src.remove(0);
        self.compile_token(token, into);

        Some(())
    }

    /// Compiles all tokens into a vector of bytecode instructions.
    pub fn compile_all(&mut self) -> Vec<BytecodeInstruction> {
        let mut instructions = BytecodeBuffer::with_capacity(self.src.len());
        while let Some(_) = self.compile_next(&mut instructions.begin()) {}
        instructions.into_instructions()
    }
}
