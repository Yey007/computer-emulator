mod register;
mod alu;
mod flag;
mod port;
mod memory;

use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::flag::Flag;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::computer::port::Port;
use crate::computer::register::Register;

pub struct Computer {
    alu: ArithmeticLogicUnit,

    accumulator: Register<4>,
    x_register: Register<4>,
    y_register: Register<4>,
    z_register: Register<4>,
    program_counter: Register<6>,

    status_flag: Flag,

    ports: [Port; 4],

    program_memory: ReadOnlyMemory,
    working_memory: ReadWriteMemory,
}

impl Computer {

}
