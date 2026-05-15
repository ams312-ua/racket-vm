use chumsky::prelude::*;
use parser::parsers::RParser;
use parser::parsers::composed::any_composed::AnyComposedParser;
use parser::parsers::primitives::AnyPrimitiveParser;
use parser::parsers::quoted::any_quoted::AnyQuotedParser;
use parser::util::print_errors;
use std::io;
use std::io::Write;
use std_native::DefaultPlugins;
use vm::vm::VM;

fn main() {
    loop {
        let mut buf = String::new();
        print!("ctest> ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        println!("Parsing: {:?}", buf.trim_end());

        let parser = parser::parser();
        let (output, errors) = parser.parse(buf.trim_end()).into_output_errors();
        if !errors.is_empty() {
            print_errors("<stdin>", &buf, errors);
        }

        let Some(tokens) = output else {
            println!("No tokens generated");
            continue;
        };

        println!("Tokens: {:#?}", tokens);

        /*let mut compiler = compiler::Compiler::new(tokens.iter().collect::<Vec<_>>());
        let all = compiler.compile_all();

        println!("Bytecode: {:#?}", all);

        let mut vm = VM::new(all.into_iter().collect());

        vm.plugins().register_collection(DefaultPlugins);
        vm.plugins().activate_namespace("base");

        let result = vm.run().unwrap();
        println!("Result: {:#?}", result.as_ref().clone());*/
    }
}
