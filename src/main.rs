#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

use std::cell::RefCell;
use crate::computer::{Computer, PROGRAM_MEMORY_SIZE};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use crate::connectable::Connectable;
use crate::connectable::device_port::DevicePort;
use crate::connectable::spliter::Spliter;
use crate::device::console::Console;
use crate::simulation::run_simulation;
use crate::un::U;

mod computer;
mod device;
mod instruction;
mod connectable;
mod un;
mod simulation;

fn load_program_from_file(path: &Path) -> Result<[u8; PROGRAM_MEMORY_SIZE], io::Error> {
    let mut f = File::open(path)?;
    let mut buf = [0_u8; PROGRAM_MEMORY_SIZE];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn main() {
    // let program = load_program_from_file(Path::new("./programs/program")).unwrap();
    let mut computer = Computer::with_program([
        0b11000001, 0b11010001, 0b00010101, 0b01110011, 0b01000100, 0b00111000, 0b10000110,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000,
    ].map(|e| (e as u8).into()));

    let mut splitter = Rc::new(RefCell::new(Spliter::<4, 2>::new()));
    let p1 = Rc::new(RefCell::new(DevicePort::<4>::new()));
    let p2 = Rc::new(RefCell::new(DevicePort::<2>::new()));
    let p3 = Rc::new(RefCell::new(DevicePort::<6>::new()));

    splitter.clone().borrow_mut().as_low_end().connect_to(p1.clone());
    p1.borrow_mut().connect_to();

    p3.borrow_mut().write(0b110101u8.into());
    let p1_val: U<4> = p1.borrow_mut().read();
    let p2_val: U<2> = p2.borrow_mut().read();

    let mut console = Console::new();

    let computer_port1 = computer.get_port(0u8.into());
    let computer_port2 = computer.get_port(1u8.into());
    let computer_pin = computer.get_pin(0u8.into());
    let ascii_port = console.ascii_port();
    let write_pin = console.write_pin();

    computer_port1.borrow_mut().connect_to(computer_port2.clone());
    computer_port2.borrow_mut().connect_to(computer_port1);

    write_pin.borrow_mut().connect_to(computer_pin.clone());
    computer_pin.borrow_mut().connect_to(write_pin);

    run_simulation(vec![Box::new(computer)]);
}
