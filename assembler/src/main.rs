mod lexer;
mod location;
mod parser;

use std::fs::File;
use std::io::Read;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = &args[1];
    let output_filename = &args[2];

    let mut input_file = match File::open(input_filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not open input file {}. Cause: {}", input_filename, err),
    };

    let mut output_file = match File::create(output_filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not create output file {}. Cause: {}", output_filename, err),
    };

    let mut input = String::new();
    match input_file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(err) => panic!("Could not read input file {}. Cause: {}", input_filename, err),
    };

    let lexer = Lexer::new(input.as_str());
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens.iter());
    let result = parser.parse();
    
    println!("hi")
}
