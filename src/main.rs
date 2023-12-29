#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

use crate::computer::{Computer, PROGRAM_MEMORY_SIZE};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use lazy_static::lazy_static;
use crate::connectable::Connectable;
use crate::connectable::device_port::DevicePort;
use crate::connectable::spliter::Spliter;
use crate::device::console::Console;
use crate::simulation::run_simulation;

mod computer;
mod device;
mod instruction;
mod connectable;
mod un;
mod simulation;
mod notif_cell;

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

    let splitter: Spliter<4, 2> = Spliter::new();
    let mut p1: DevicePort<4> = DevicePort::new();
    let mut p2: DevicePort<2> = DevicePort::new();
    let mut p3: DevicePort<6> = DevicePort::new();

    // p1.connect_to(&splitter.as_low_end());
    // p2.connect_to(&splitter.as_high_end());
    p3.connect_to(&splitter);

    // let mut console = Console::new();
    //
    // let computer_port1 = computer.get_port(0u8.into());
    // let computer_port2 = computer.get_port(1u8.into());
    // let computer_pin = computer.get_pin(0u8.into());
    // let ascii_port = console.ascii_port();
    // let write_pin = console.write_pin();
    //
    // computer_port1.connect_to(computer_port2);
    // computer_port2.connect_to(computer_port1);
    //
    // write_pin.connect_to(computer_pin);
    //
    run_simulation(vec![Box::new(computer)]);
}
