use crate::computer::{Computer, PROGRAM_MEMORY_SIZE};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

mod computer;
mod instruction;

fn load_program_from_file(path: &Path) -> Result<[u8; PROGRAM_MEMORY_SIZE], io::Error> {
    let mut f = File::open(path)?;
    let mut buf = [0_u8; PROGRAM_MEMORY_SIZE];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn main() {
    let program = load_program_from_file(Path::new("./programs/program")).unwrap();
    let computer = Computer::with_program(program);
    computer.run()
}
