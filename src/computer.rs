mod register;
mod alu;
mod flag;
mod port;
mod memory;

use ux::{u4, u6};
use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::flag::Flag;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::computer::port::Port;
use crate::computer::register::Register;

pub struct Computer {
    alu: ArithmeticLogicUnit,

    accumulator: Register<u4>,
    x_register: Register<u4>,
    y_register: Register<u4>,
    z_register: Register<u4>,
    program_counter: Register<u6>,

    status_flag: Flag,

    ports: [Port; 4],

    program_memory: ReadOnlyMemory,
    working_memory: ReadWriteMemory,
}

impl Computer {
}
