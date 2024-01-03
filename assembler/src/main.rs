use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = &args[1];
    let output_filename = &args[2];

    let input_file = match File::open(input_filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not open input file {}. Cause: {}", input_filename, err),
    };

    let output_file = match File::create(output_filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not create output file {}. Cause: {}", output_filename, err),
    };
}
