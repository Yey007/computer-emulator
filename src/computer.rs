mod alu;
mod memory;
mod register;

use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::port::device_port::DevicePort;
use crate::computer::register::Register;
use crate::instruction::{decode_instruction, Instruction};
use crate::types::{InstructionBitType, PinIndex, PINS_SIZE, PortIndex, PORT_BITS, PORTS_SIZE, PROGRAM_MEMORY_SIZE, ProgramCounterType, RegisterIndex, WORKING_MEMORY_SIZE, WorkingType};
use std::borrow::{BorrowMut};
use crate::port::device_pin::DevicePin;
use crate::port::Port;

pub struct Computer {
    alu: ArithmeticLogicUnit,

    x_register: Register<WorkingType>,
    y_register: Register<WorkingType>,
    z_register: Register<WorkingType>,
    program_counter: Register<ProgramCounterType>,

    status_flag: bool,

    ports: [DevicePort<PORT_BITS>; PORTS_SIZE],
    pins: [DevicePin; PINS_SIZE],

    program_memory: ReadOnlyMemory<InstructionBitType, PROGRAM_MEMORY_SIZE>,
    working_memory: ReadWriteMemory<WorkingType, WORKING_MEMORY_SIZE>,
}

impl Computer {
    pub fn with_program(program: [InstructionBitType; PROGRAM_MEMORY_SIZE]) -> Self {
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

    fn fetch(&self) -> InstructionBitType {
        let addr = self.program_counter.load();
        self.program_memory.read(addr)
    }

    fn decode(&self, instruction: InstructionBitType) -> Instruction {
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
            Instruction::SEP { pin_id } => self.get_pin(pin_id).write(true),
            Instruction::RSP { pin_id } => self.get_pin(pin_id).write(false),
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

    fn get_register(&mut self, id: RegisterIndex) -> &mut Register<WorkingType> {
        let id_u8: u8 = id.into();
        match id_u8 {
            0 => self.alu.accumulator_mut(),
            1 => self.x_register.borrow_mut(),
            2 => self.y_register.borrow_mut(),
            3 => self.z_register.borrow_mut(),
            _ => panic!("Max value of u2 exceeded."),
        }
    }

    fn get_port(&mut self, id: PortIndex) -> &mut DevicePort<WorkingType> {
        let id_u8: u8 = id.into();
        self.ports[id_u8 as usize].borrow_mut()
    }

    fn get_pin(&mut self, id: PinIndex) -> &mut DevicePin {
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
