mod alu;
mod memory;
mod register;

use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::port::device_port::DevicePort;
use crate::computer::register::Register;
use crate::instruction::{decode_instruction, Instruction};
use std::borrow::{BorrowMut};
use crate::port::device_pin::DevicePin;
use crate::port::Port;
use crate::un::U;

pub const WORKING_BITS: usize = 4;
pub const PC_BITS: usize = 6;
pub const INSTRUCTION_BITS: usize = 8;
pub const PORT_BITS: usize = 4;

pub const NUM_REGISTERS: usize = 4;
pub const NUM_PORTS: usize = 4;
pub const NUM_PINS: usize = 8;

pub const REGISTER_INDEX_BITS: usize = NUM_REGISTERS.ilog2() as usize;
pub const PORT_INDEX_BITS: usize = NUM_PORTS.ilog2() as usize;
pub const PIN_INDEX_BITS: usize = NUM_PORTS.ilog2() as usize;

pub const PROGRAM_MEMORY_SIZE: usize = 2usize.pow(PC_BITS as u32);
pub const WORKING_MEMORY_SIZE: usize = 2usize.pow(2 * WORKING_BITS as u32);  // two registers used to index

pub struct Computer {
    alu: ArithmeticLogicUnit,

    x_register: Register<WORKING_BITS>,
    y_register: Register<WORKING_BITS>,
    z_register: Register<WORKING_BITS>,
    program_counter: Register<PC_BITS>,

    status_flag: bool,

    ports: [DevicePort<PORT_BITS>; NUM_PORTS],
    pins: [DevicePin; NUM_PINS],

    program_memory: ReadOnlyMemory<INSTRUCTION_BITS, PROGRAM_MEMORY_SIZE>,
    working_memory: ReadWriteMemory<WORKING_BITS, WORKING_MEMORY_SIZE>,
}

impl Computer {
    pub fn with_program(program: [U<INSTRUCTION_BITS>; PROGRAM_MEMORY_SIZE]) -> Self {
        Computer {
            alu: ArithmeticLogicUnit::new(),
            x_register: Register::new(),
            y_register: Register::new(),
            z_register: Register::new(),
            program_counter: Register::new(),
            status_flag: false,
            ports: [DevicePort::new(), DevicePort::new(), DevicePort::new(), DevicePort::new()],
            pins: [
                DevicePin::new(), DevicePin::new(), DevicePin::new(), DevicePin::new(),
                DevicePin::new(), DevicePin::new(), DevicePin::new(), DevicePin::new()
            ],
            program_memory: ReadOnlyMemory::with_values(program),
            working_memory: ReadWriteMemory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst_bits = self.fetch();
            let inst = self.decode(inst_bits);
            self.execute(inst);
            self.program_counter.increment();
        }
    }

    fn fetch(&self) -> U<INSTRUCTION_BITS> {
        let addr = self.program_counter.load();
        self.program_memory.read(addr)
    }

    fn decode(&self, instruction: U<INSTRUCTION_BITS>) -> Instruction {
        decode_instruction(instruction)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => (),
            Instruction::STR { register_id } => {
                let addr = self.decode_xy();
                let register = self.get_register(register_id);
                let value = register.load();
                self.working_memory.write(addr.into(), value)
            }
            Instruction::LOD { register_id } => {
                let addr = self.decode_xy();
                let value = self.working_memory.read(addr.into());
                let register = self.get_register(register_id);
                register.store(value)
            }
            Instruction::LDI {
                register_id,
                immediate,
            } => {
                let register = self.get_register(register_id);
                register.store(immediate)
            }
            Instruction::INC { register_id } => {
                let register = self.get_register(register_id);
                register.increment()
            }
            Instruction::DEC { register_id } => {
                let register = self.get_register(register_id);
                register.decrement()
            }
            Instruction::MOV {
                register_to_id,
                register_from_id,
            } => {
                let register_to = self.get_register(register_to_id);
                let value = register_to.load();
                let register_from = self.get_register(register_from_id);
                register_from.store(value)
            }
            Instruction::INP { port_id } => {
                let value = self.get_port(port_id).read();
                self.z_register.store(value)
            }
            Instruction::OUT { port_id } => {
                let val = self.z_register.load();
                let port = self.get_port(port_id);
                port.write(val)
            }
            Instruction::SEP { pin_id } => self.get_pin(pin_id).write(1u8.into()),
            Instruction::RSP { pin_id } => self.get_pin(pin_id).write(0u8.into()),
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

    fn get_register(&mut self, id: U<REGISTER_INDEX_BITS>) -> &mut Register<WORKING_BITS> {
        let id_u8: u8 = id.into();
        match id_u8 {
            0 => self.alu.accumulator_mut(),
            1 => self.x_register.borrow_mut(),
            2 => self.y_register.borrow_mut(),
            3 => self.z_register.borrow_mut(),
            _ => panic!("Max value of u2 exceeded."),
        }
    }

    fn get_port(&mut self, id: U<PORT_INDEX_BITS>) -> &mut DevicePort<WORKING_BITS> {
        let id_u8: u8 = id.into();
        self.ports[id_u8 as usize].borrow_mut()
    }

    fn get_pin(&mut self, id: U<PIN_INDEX_BITS>) -> &mut DevicePin {
        let id_u8: u8 = id.into();
        self.pins[id_u8 as usize].borrow_mut()
    }

    fn decode_xy(&self) -> u8 {
        let x: u8 = self.x_register.load().into();
        let y: u8 = self.y_register.load().into();

        let x_shifted = x << (u8::BITS / 2);
        x_shifted | y
    }
}
