use chumsky::Parser as ChumskyParser;
use clap::{Parser, Subcommand};
use compiler::Compiler;
use parser::util::print_errors;
use std_native::DefaultPlugins;
use std::{error::Error, fs::File, path::PathBuf};

use vm::vm::VM;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: SubCommand,
    #[arg(short = 'p', long)]
    show_parsed: bool,
    #[arg(short = 'b', long)]
    show_bytecode: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Run { file: PathBuf },
    Repl,
}

impl Cli {
    pub fn handle() -> Result<(), Box<dyn Error>> {
        let this = Self::parse();
        match &this.command {
            SubCommand::Run { file } => this.run(file),
            SubCommand::Repl => this.repl(),
        }
    }

    fn run(&self, file: &PathBuf) -> Result<(), Box<dyn Error>> {
        if !file.is_file() {
            return Err(format!("File {:?} does not exist", file).into());
        }

        let Some(filename) = file.file_name().unwrap().to_str() else {
            return Err(format!("Invalid file provided: {:?}", file).into());
        };

        let file_content = std::fs::read_to_string(file)?
            .lines()
            .filter(|l| !l.trim_start().starts_with(';')) // remove comments
            .filter(|l| !l.trim_start().starts_with("#lang")) // we skip the #lang line since it is not needed for the vm and just adds noise to the bytecode
            .collect::<String>();

        self.run_vm(filename, &file_content)
    }

    fn repl(&self) -> Result<(), Box<dyn Error>> {
        use std::io::Write;

        loop {
            let mut buf = String::new();
            print!("repl> ");
            std::io::stdout().flush()?;
            if std::io::stdin().read_line(&mut buf).is_err() {
                break;
            }

            #[cfg(debug_assertions)]
            println!("Parsing: {:?}", buf.trim_end());

            self.run_vm("<stdin>", buf.trim_end())?;
        }

        Ok(())
    }

    fn run_vm(&self, filename: &str, code: &str) -> Result<(), Box<dyn Error>> {
        let parser = parser::parser();

        let (output, errors) = parser.parse(code.trim_end()).into_output_errors();
        if !errors.is_empty() {
            print_errors(filename, &code, errors);
        }

        let Some(tokens) = output else {
            return Err("No tokens produced".into());
        };

        if self.show_parsed {
            println!("Tokens: {:#?}", tokens);
        }

        // Compilar el codigo
        let mut compiler = Compiler::new(tokens.iter().collect());
        let compiled = compiler.compile_all();

        if self.show_bytecode {
            println!("Bytecode: {:#?}", compiled);
        }

        let mut vm = VM::new(compiled);
        vm.plugins().register_collection(DefaultPlugins); // registrar los plugins por defecto
        vm.plugins().activate_namespace("base"); // activar el namespace base
        
        let result = vm.run()?;
        println!("{}", result.as_ref());

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Cli::handle()
}
