use std::io;
use std::io::Write;
use chumsky::prelude::*;
use parser::parsers::composed::any_composed::AnyComposedParser;
use parser::parsers::primitives::AnyPrimitiveParser;
use parser::parsers::quoted::any_quoted::AnyQuotedParser;
use parser::parsers::RParser;
use parser::util::print_errors;

fn main() {
    loop {
        let mut buf = String::new();
        print!("ctest> ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        println!("Parsing: {:?}", buf.trim_end());

        let parser = choice((
            AnyQuotedParser::token_parser(),
            AnyComposedParser::token_parser(),
            AnyPrimitiveParser::token_parser(),
        ));
        let (output, errors) = parser.parse(buf.trim_end()).into_output_errors();
        if !errors.is_empty() {
            print_errors("<stdin>", &buf, errors);
        }
        match output {
            Some(token) => println!("{:#?}", token),
            None => eprintln!("Failed to parse input"),
        }
    }
}
