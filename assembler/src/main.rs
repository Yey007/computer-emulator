mod lexer;
mod location;

use std::fs::File;
use std::io::Read;

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

    let mut lexer = lexer::Lexer::new(input);
    lexer.lex();

    println!("hi")
}
