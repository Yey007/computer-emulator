#![feature(if_let_guard)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

mod lexer;
mod location;
mod parser;
mod codegen;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::lexer::{Lexer, TokenKind};
use crate::parser::{ErrorTokenKind, ParseError, ParseErrorKind, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = Path::new(args.get(1).expect("Input file is required")).to_path_buf();
    let output_filename = match args.get(2) {
        None => input_filename.with_extension("out"),
        Some(f) => Path::new(f).to_path_buf()
    };

    let mut input_file = match File::open(input_filename.clone()) {
        Ok(file) => file,
        Err(err) => panic!("Could not open input file {}. Cause: {}", input_filename.display(), err),
    };

    let mut output_file = match File::create(output_filename.clone()) {
        Ok(file) => file,
        Err(err) => panic!("Could not create output file {}. Cause: {}", output_filename.display(), err),
    };

    let mut input = String::new();
    match input_file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(err) => panic!("Could not read input file {}. Cause: {}", input_filename.display(), err),
    };

    let mut lexer = Lexer::new(input.as_str());

    let mut parser = Parser::new(lexer.iter());
    let result = parser.parse();

    if let Err(errors) = result {
        report_errors(&input_filename, errors);
    }
}

fn report_errors(file: &PathBuf, errors: Vec<ParseError>) {
    for error in errors {
        let location = error.token
            .clone()
            .map(|t| format!("{}:{}", t.location.line, t.location.col))
            .unwrap_or("eof".to_owned());

        print!("Error in {} at {}: ", file.display(), location);

        match error {
            ParseError { kind: ParseErrorKind::UnexpectedToken { expected_types }, token: Some(token), .. } => {
                let expected = expected_types.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ");
                let found = token.kind;
                let text = token.text;
                print!("Expected one of {expected} but found {found} ");

                if found != TokenKind::Newline && found != TokenKind::Colon {
                    println!("'{text}'");
                } else {
                    println!();
                }
            },
            ParseError { kind: ParseErrorKind::UnexpectedToken { expected_types }, token: None, .. } => {
                let expected = expected_types.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ");
                println!("Expected one of {expected} but found eof")
            }
            ParseError { kind: ParseErrorKind::InvalidNumberLiteral { cause }, token: Some(token), .. } => {
                println!("Invalid number literal '{}'. Cause: {}", token.text, cause)
            },
            _ => {
                println!("Unknown error")
            }
        };

        if let Some(help) = error.help {
            println!("Help: {help}")
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            TokenKind::Newline => "'\\n'",
            TokenKind::Colon => "':'",
            TokenKind::LabelIdentifier => "label identifier",
            TokenKind::Instruction { .. } => "instruction",
            TokenKind::NumberLiteral { .. } => "number literal",
            TokenKind::RegisterLiteral { .. } => "register literal"
        };
        write!(f, "{}", repr)
    }
}

impl Display for ErrorTokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ErrorTokenKind::Newline => "'\\n'",
            ErrorTokenKind::Colon => "':'",
            ErrorTokenKind::LabelIdentifier => "label identifier",
            ErrorTokenKind::Instruction => "instruction",
            ErrorTokenKind::NumberLiteral => "number literal",
            ErrorTokenKind::RegisterLiteral => "register literal"
        };
        write!(f, "{}", repr)
    }
}
