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
use std::borrow::{Borrow, BorrowMut};
use ux::{u2, u4, u6};

pub const PROGRAM_MEMORY_SIZE: usize = 2_usize.pow(6);
pub const WORKING_MEMORY_SIZE: usize = 2_usize.pow(8);

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
            working_memory: ReadWriteMemory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst_bits = self.fetch();
            let inst = self.decode(inst_bits);
            self.execute(inst);
            self.program_counter.increment()
        }
    }

    fn fetch(&self) -> u8 {
        let addr = self.program_counter.load();
        self.program_memory.read(addr)
    }

    fn decode(&self, instruction: u8) -> Instruction {
        decode_instruction(instruction)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => (),
            Instruction::STR { register_id } => {
                let addr = self.decode_xy();
                let register = self.get_register(register_id);
                let value = register.load();
                self.working_memory.write(addr, value)
            }
            Instruction::LOD { register_id } => {
                let addr = self.decode_xy();
                let value = self.working_memory.read(addr);
                let register = self.get_register_mut(register_id);
                register.store(value)
            }
            Instruction::LDI {
                register_id,
                immediate,
            } => {
                let register = self.get_register_mut(register_id);
                register.store(immediate)
            }
            Instruction::INC { register_id } => {
                let register = self.get_register_mut(register_id);
                register.increment()
            }
            Instruction::DEC { register_id } => {
                let register = self.get_register_mut(register_id);
                register.decrement()
            }
            Instruction::INP {
                register_id,
                port_id,
            } => {
                let value = self.get_port(port_id).read();
                let register = self.get_register_mut(register_id);
                register.store(value)
            }
            Instruction::OUT {
                register_id,
                port_id,
            } => {
                let value = self.get_register(register_id).load();
                let port = self.get_port_mut(port_id);
                port.write(value)
            }
            Instruction::ADD { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.add(value)
            }
            Instruction::SUB { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.sub(value)
            }
            Instruction::BOR { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.bor(value)
            }
            Instruction::AND { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.and(value)
            }
            Instruction::CMP { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.cmp(value)
            }
            Instruction::GRT { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.grt(value)
            }
            Instruction::LES { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.les(value)
            }
            Instruction::BRN { immediate } => self.program_counter.store(immediate),
            Instruction::SSF => self.status_flag = true,
            Instruction::RSF => self.status_flag = false,
        }
    }

    fn get_register(&self, id: u2) -> &Register<u4> {
        let id_u8: u8 = id.into();
        match id_u8 as usize {
            0 => self.alu.accumulator(),
            1 => self.x_register.borrow(),
            2 => self.y_register.borrow(),
            3 => self.z_register.borrow(),
            _ => panic!("I'm sorry, what? Max value of u2 exceeded."),
        }
    }

    fn get_port(&self, id: u2) -> &Port<u4> {
        let id_u8: u8 = id.into();
        self.ports[id_u8 as usize].borrow()
    }

    fn get_register_mut(&mut self, id: u2) -> &mut Register<u4> {
        let id_u8: u8 = id.into();
        match id_u8 as usize {
            0 => self.alu.accumulator_mut(),
            1 => self.x_register.borrow_mut(),
            2 => self.y_register.borrow_mut(),
            3 => self.z_register.borrow_mut(),
            _ => panic!("I'm sorry, what? Max value of u2 exceeded."),
        }
    }

    fn get_port_mut(&mut self, id: u2) -> &mut Port<u4> {
        let id_u8: u8 = id.into();
        self.ports[id_u8 as usize].borrow_mut()
    }

    fn decode_xy(&self) -> u8 {
        let x: u8 = self.x_register.load().into();
        let y: u8 = self.y_register.load().into();

        let x_shifted = x << (u8::BITS / 2);
        x_shifted | y
    }
}
