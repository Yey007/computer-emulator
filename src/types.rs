use ux::{u2, u4, u6};

pub type WorkingType = u4;
pub type ProgramCounterType = u6;

pub type InstructionBitType = u8;

pub type RegisterIndexType = u2;
pub type PortIndexType = u2;

pub const PROGRAM_MEMORY_SIZE: usize = 2_usize.pow(6);
pub const WORKING_MEMORY_SIZE: usize = 2_usize.pow(8);
pub const PORTS_SIZE: usize = 2_usize.pow(2);
