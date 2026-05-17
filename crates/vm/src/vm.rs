use std::collections::VecDeque;

use common::{
    bytecode::BytecodeInstruction,
    value::{self, GCValue, Value},
};
use rapidhash::{HashMapExt, RapidHashMap};

use crate::{
    control::{self, ControlFrame},
    error::Error,
    frame::{self, Frame},
    function::DefinedFunction,
    native::NativePlugins,
    stack::Stack,
};

pub struct VM {
    global_stack: Stack<GCValue>,
    frames: Stack<Frame>,
    native_plugins: NativePlugins,
    /// Maps function names to their definitions.
    ///
    /// No newtype is defined due to define function variant on value
    /// being the same size as the whole enumeration, so it does not
    /// save any space.
    defined_functions: RapidHashMap<Box<str>, DefinedFunction>,
    /// Stack of control frames that are used on control flow OPs
    control_frames: Vec<ControlFrame>,
    /// Marker to give id's to calls, used to store their results in the call_results map
    next_call_marker: usize,
    /// Map of call markers to their results, used to store results of calls until they are needed on native calls
    call_results: RapidHashMap<usize, GCValue>,
}

impl VM {
    pub fn new(bytecode: Vec<BytecodeInstruction>) -> Self {
        Self {
            global_stack: Stack::new(),
            frames: Stack::from(Frame::new(bytecode)),
            native_plugins: NativePlugins::new(),
            defined_functions: RapidHashMap::new(),
            control_frames: Vec::new(),
            next_call_marker: 0,
            call_results: RapidHashMap::new(),
        }
    }

    pub fn plugins(&mut self) -> &mut NativePlugins {
        &mut self.native_plugins
    }

    /// Takes a given value and relates it to a variable if it is an identifier.
    ///
    /// The relation process starts with the given frame and goes up the frame stack until it finds a variable with the same name as the identifier, returning its value. If it does not find such a variable, it returns an error.
    ///
    /// If it's not an identifier, just returns the value.
    fn try_lookup_value(&self, frame: &Frame, value: Value) -> Result<GCValue, Error> {
        if let Value::Identifier(name) = value {
            if let Some(value) = frame.locals.get(&name) {
                return Ok(value.clone());
            };

            for frame in self.frames.inner_vec().iter().rev() {
                if let Some(value) = frame.locals.get(&name) {
                    return Ok(value.clone());
                }
            }

            if self.defined_functions.contains_key(&name)
                || self.native_plugins.get_plugin(&name).is_some()
            {
                return Ok(GCValue::new(Value::Function(name)));
            }

            Err(Error::UndefinedVariable(name.to_string()))
        } else {
            Ok(GCValue::new(value))
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn make_frame(&self, bytecode: Vec<BytecodeInstruction>) -> Frame {
        self.frames
            .last()
            .expect("frame exists")
            .make_child(bytecode)
    }

    fn step(&mut self) -> Result<bool, Error> {
        use BytecodeInstruction::*;

        let Some(mut frame) = self.pop_frame() else {
            return Ok(false); // No frames remaining, stop the VM
        };

        let Some(next_instruction) = frame.next_instruction() else {
            // Call callback
            let res = self.global_stack.pop().unwrap_or(GCValue::new(Value::Void));
            self.frame_done(res.clone());
            return Ok(true); // Maybe frames remaining, so dont stop yet.
        };

        match next_instruction {
            Push(v) => {
                let v = self.try_lookup_value(&frame, v)?;
                self.global_stack.push(v)
            }
            Pop => {
                self.global_stack.pop();
            }
            Call { function, argc } => {
                let mut args = Vec::with_capacity(argc);
                self.global_stack.pop_many_into(argc, &mut args);

                let fn_name = if let Some(function) = function {
                    function.clone()
                } else {
                    self.global_stack
                        .pop()
                        .expect("Function name on stack")
                        .as_string()?
                        .into()
                };

                if let Some(defined_function) = self.defined_functions.get(&fn_name) {
                    let bytecode = defined_function.bytecode.clone();
                    let mut next_frame = frame.make_child(bytecode);

                    if args.len() < defined_function.args.len() {
                        return Err(Error::ArityMismatch {
                            expected: defined_function.args.len(),
                            variadic: defined_function.variadic_arg.is_some(),
                            got: args.len(),
                        });
                    }

                    // Add arguments to the next frame. No need to check for bounds cuz we already did
                    for (i, arg) in defined_function.args.iter().enumerate() {
                        next_frame.locals.insert(arg.clone(), args[i].clone());
                    }

                    // for variadic, take the rest and into a list
                    if let Some(variadic_arg) = &defined_function.variadic_arg {
                        let variadic_args = args.drain(defined_function.args.len()..).collect();
                        next_frame.locals.insert(
                            variadic_arg.clone(),
                            GCValue::new(Value::list(variadic_args)),
                        );
                    }

                    // push frames, the next call to step will execute the new frame we just pushed
                    self.push_frame(frame);
                    self.push_frame(next_frame);

                    // early return to avoid pushing twice
                    return Ok(true);
                } else if let Some(native_fn) = self.native_plugins.get_plugin(&fn_name) {
                    use crate::plugin::{MaybeGcValue, NativeError};
                    let result = match native_fn.try_call(self, &args) {
                        Ok(result) => Ok(result),
                        Err(e) => match e {
                            NativeError::VmError(e) => Err(*e),
                            other => return Err(Error::Native(other)),
                        },
                    }?;

                    match result {
                        MaybeGcValue::Value(v) => self.global_stack.push(GCValue::new(v)),
                        MaybeGcValue::Gc(v) => self.global_stack.push(v),
                    };
                } else {
                    return Err(Error::UndefinedFunction(fn_name.into()));
                }
            }
            Define { name, value } => {
                frame
                    .locals
                    .insert(name.clone(), GCValue::new(value.clone()));
            }
            DefineFunction {
                name,
                args,
                variadic_arg,
                bytecode,
            } => {
                self.defined_functions.insert(
                    name.clone(),
                    DefinedFunction {
                        name: name.clone(),
                        args: args.clone(),
                        variadic_arg: variadic_arg.clone(),
                        bytecode: bytecode.clone(),
                    },
                );
            }
            Undefine { name } => {
                frame.locals.remove(&name);
                self.defined_functions.remove(&name);
            }
            If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.control_frames.push(ControlFrame::IfSelect {
                    then_branch: then_branch,
                    else_branch: else_branch,
                });
                let next_frame = frame.make_child(condition.clone());
                self.push_frame(frame);
                self.push_frame(next_frame);

                // early return to avoid pushing twice
                return Ok(true);
            }
            Cond {
                conditions,
                branches,
                else_branch,
            } => {
                let next_frame = frame.make_child(conditions[0].clone());

                self.control_frames.push(ControlFrame::CondSelect {
                    next_index: 0,
                    conditions: conditions,
                    branches: branches,
                    else_branch: else_branch,
                });
                
                self.push_frame(frame);
                self.push_frame(next_frame);

                // early return to avoid pushing twice
                return Ok(true);
            }
            _ => todo!(),
        }

        // Not finished with the frame, so restore it
        self.push_frame(frame);
        Ok(true)
    }

    fn frame_done(&mut self, result: GCValue) {
        use ControlFrame::*;

        let Some(control_frame) = self.control_frames.pop() else {
            // No control frame, just push result to global stack
            self.global_stack.push(result);
            return;
        };

        match control_frame {
            IfSelect {
                then_branch,
                else_branch,
            } => {
                if result.is_truthy() {
                    self.frames.push(self.make_frame(then_branch));
                } else {
                    self.frames.push(self.make_frame(else_branch));
                }
            }
            CondSelect {
                next_index,
                conditions,
                branches,
                else_branch,
            } => {
                if result.is_truthy() {
                    self.frames
                        .push(self.make_frame(branches[next_index].clone()));

                    return;
                }

                let next_candidate = next_index + 1;
                if next_candidate < conditions.len() {
                    self.frames
                        .push(self.make_frame(conditions[next_candidate].clone()));
                    
                    self.control_frames.push(CondSelect {
                        next_index: next_candidate,
                        conditions,
                        branches,
                        else_branch,
                    });
                } else {
                    if let Some(else_branch) = else_branch {
                        self.frames.push(self.make_frame(else_branch));
                    }
                }
            }
            CallResult { marker } => {
                self.call_results.insert(marker, result);
            }
        }
    }

    pub fn call(&mut self, function: Box<str>, args: Vec<GCValue>) -> Result<GCValue, Error> {
        let call_frame = Frame::new(vec![BytecodeInstruction::Call {
            function: Some(function),
            argc: args.len(),
        }]);
        for arg in args.into_iter().rev() {
            self.global_stack.push(arg);
        }
        self.push_frame(call_frame);

        let marker = self.next_call_marker;
        self.next_call_marker += 1;
        self.control_frames
            .push(ControlFrame::CallResult { marker });

        while self.step()? {
            if let Some(result) = self.call_results.remove(&marker) {
                return Ok(result);
            }
        }

        // We here, that means the VM either halted or ran out of frames without returning a result for the call, which should not happen, so we return an error.
        Err(Error::VMHalted)
    }

    pub fn run(&mut self) -> Result<GCValue, Error> {
        /*let base_frame = self.base_frame.take().unwrap();
        self.run_frame(base_frame)*/

        while self.step()? {}

        Ok(self
            .global_stack
            .pop()
            .unwrap_or_else(|| GCValue::new(Value::Void)))
    }
}
