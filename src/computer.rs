mod alu;
mod memory;
mod port;
mod register;

use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::computer::port::Port;
use crate::computer::register::Register;
use crate::instruction::{decode_instruction, Instruction};
use std::borrow::Borrow;
use ux::{u2, u4, u6};

type ProgramMemoryAddr = u6;
type WorkingMemoryAddr = u8;

pub const PROGRAM_MEMORY_SIZE: usize = 2_usize.pow(6);
pub const WORKING_MEMORY_SIZE: usize = 2_usize.pow(WorkingMemoryAddr::BITS);

pub struct Computer {
    alu: ArithmeticLogicUnit,

    x_register: Register<u4>,
    y_register: Register<u4>,
    z_register: Register<u4>,
    program_counter: Register<u6>,

    status_flag: bool,

    ports: [Port<u4>; 4],

    program_memory: ReadOnlyMemory<u8, ProgramMemoryAddr, PROGRAM_MEMORY_SIZE>,
    working_memory: ReadWriteMemory<u4, WorkingMemoryAddr, WORKING_MEMORY_SIZE>,
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
            working_memory: ReadWriteMemory::new(),
        }
    }

    pub fn run(&self) {
        loop {
            let inst_bits = self.fetch();
            let inst = self.decode(inst_bits);
            self.execute(inst)
        }
    }

    fn fetch(&self) -> u8 {
        let addr = self.program_counter.value;
        self.program_memory.read(addr)
    }

    fn decode(&self, instruction: u8) -> Instruction {
        decode_instruction(instruction)
    }

    fn execute(&self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => (),
            Instruction::STR {
                register: registerId,
            } => {
                let mut register = self.get_register(registerId);
                let addr = self.decode_xy();
                register.store(self.working_memory.read(addr))
            }
            Instruction::LOD {
                register: registerId,
            } => {
                let register = self.get_register(registerId);
            }
        }
    }

    fn get_register(&self, id: u2) -> &Register<u4> {
        match id.into() {
            0 => self.alu.accumulator(),
            1 => self.x_register.borrow(),
            2 => self.y_register.borrow(),
            3 => self.z_register.borrow(),
            _ => panic!("I'm sorry, what? Max value of u2 exceeded."),
        }
    }

    fn decode_xy(&self) -> WorkingMemoryAddr {
        let x: WorkingMemoryAddr = self.x_register.load().into();
        let y: WorkingMemoryAddr = self.y_register.load().into();

        let x_shifted = x << (WorkingMemoryAddr::BITS / 2);
        x_shifted | y
    }
}
