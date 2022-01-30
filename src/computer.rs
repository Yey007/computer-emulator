mod register;
mod alu;
mod port;
mod memory;

use ux::{u4, u6};
use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::computer::port::Port;
use crate::computer::register::Register;

const PROGRAM_MEMORY_SIZE: usize = 2_usize.pow(6);
const WORKING_MEMORY_SIZE: usize = 2_usize.pow(8);

pub struct Computer {
    alu: ArithmeticLogicUnit,

    x_register: Register<u4>,
    y_register: Register<u4>,
    z_register: Register<u4>,
    program_counter: Register<u6>,

    status_flag: bool,

    ports: [Port<u4>; 4],

    program_memory: ReadOnlyMemory<u8, PROGRAM_MEMORY_SIZE>,
    working_memory: ReadWriteMemory<u4, WORKING_MEMORY_SIZE>,
}

impl Computer {
    pub fn with_program(program: [u8; PROGRAM_MEMORY_SIZE]) -> Self {
        Computer {
            alu: ArithmeticLogicUnit::new(),
            x_register: Register::new(),
            y_register: Register::new(),
            z_register: Register::new(),
            program_counter: Register::new(),
            status_flag: false,
            ports: [Port::new(), Port::new(), Port::new(), Port::new()],
            program_memory: ReadOnlyMemory::with_values(program),
            working_memory: ReadWriteMemory::new()
        }
    }
}
