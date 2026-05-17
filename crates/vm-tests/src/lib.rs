use std::error::Error;

use chumsky::Parser;
use compiler::Compiler;
use std_native::DefaultPlugins;
use vm::vm::VM;

pub(crate) fn build_vm_for(test_name: &str, code: &str) -> Result<VM, Box<dyn Error>> {
    let parser = parser::parser();
    
    let (output, errors) = parser.parse(code.trim_end()).into_output_errors();
    if !errors.is_empty() {
        parser::util::print_errors(&format!("<vm-test-{test_name}>"), &code, errors);
    }

    let Some(tokens) = output else {
        return Err(format!("Failed to parse code for test {test_name}").into());
    };

    let compiled = Compiler::new(tokens.iter().collect()).compile_all();

    let mut vm = VM::new(compiled);

    vm.plugins().register_collection(DefaultPlugins);
    vm.plugins().activate_namespace("base");

    Ok(vm)
}