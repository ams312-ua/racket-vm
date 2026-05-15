use chumsky::Parser as ChumskyParser;
use clap::{Parser, Subcommand};
use parser::util::print_errors;
use std::{error::Error, fs::File, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: SubCommand,
    #[arg(short, long)]
    show_parsed: bool,
    #[arg(short, long)]
    show_bytecode: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Run { file: PathBuf },
}

impl Cli {
    pub fn handle() -> Result<(), Box<dyn Error>> {
        let this = Self::parse();
        match &this.command {
            SubCommand::Run { file } => this.run(file),
        }
    }

    fn run(&self, file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = std::fs::read_to_string(file)?
            .lines()
            .filter(|l| l.trim_start().starts_with(';')) // remove comments
            .collect::<String>();

        let parser = parser::parser();

        let (output, errors) = parser.parse(file.trim_end()).into_output_errors();
        if !errors.is_empty() {
            print_errors("<stdin>", &file, errors);
        }

        Ok(())
    }
}
