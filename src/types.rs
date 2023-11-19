use ux::{u2, u3, u4, u6};

pub type WorkingType = u4;
pub type ProgramCounterType = u6;

pub type InstructionBitType = u8;

pub type RegisterIndex = u2;
pub type PortIndex = u2;
pub type PinIndex = u3;

pub const PROGRAM_MEMORY_SIZE: usize = 2_usize.pow(6);
pub const WORKING_MEMORY_SIZE: usize = 2_usize.pow(8);
pub const PORT_BITS: usize = 4;
pub const PORTS_SIZE: usize = 2_usize.pow(2);
pub const PINS_SIZE: usize = 2_usize.pow(3);
