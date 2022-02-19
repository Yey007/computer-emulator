use bitmatch::bitmatch;
use ux::{u2, u4, u6};

#[derive(Debug)]
pub enum Instruction {
    NOP,
    STR { register: u2 },
    LOD { register: u2 },
    LDI { register: u2, immediate: u4 },
    INC { register: u2 },
    DEC { register: u2 },
    INP { register: u2, port: u2 },
    OUT { register: u2, port: u2 },
    ADD { register: u2 },
    SUB { register: u2 },
    NEG { register: u2 },
    BOR { register: u2 },
    AND { register: u2 },
    CMP { register: u2 },
    GRT { register: u2 },
    LES { register: u2 },
    BRN { immediate: u6 },
    SSF,
    RSF,
}

#[bitmatch]
pub fn decode_instruction(binary: u8) -> Instruction {
    #[bitmatch]
    match binary {
        "000001rr" => Instruction::STR {
            register: u2::new(r),
        },
        "000010rr" => Instruction::LOD {
            register: u2::new(r),
        },
        "11rrxxxx" => Instruction::LDI {
            register: u2::new(r),
            immediate: u4::new(x),
        },
        "000011rr" => Instruction::INC {
            register: u2::new(r),
        },
        "000100rr" => Instruction::DEC {
            register: u2::new(r),
        },
        "0100rrpp" => Instruction::INP {
            register: u2::new(r),
            port: u2::new(p),
        },
        "0101rrpp" => Instruction::OUT {
            register: u2::new(r),
            port: u2::new(p),
        },
        "000101rr" => Instruction::ADD {
            register: u2::new(r),
        },
        "000110rr" => Instruction::SUB {
            register: u2::new(r),
        },
        "000111rr" => Instruction::NEG {
            register: u2::new(r),
        },
        "001000rr" => Instruction::BOR {
            register: u2::new(r),
        },
        "001001rr" => Instruction::AND {
            register: u2::new(r),
        },
        "001011rr" => Instruction::CMP {
            register: u2::new(r),
        },
        "001100rr" => Instruction::GRT {
            register: u2::new(r),
        },
        "001101rr" => Instruction::LES {
            register: u2::new(r),
        },
        "10xxxxxx" => Instruction::BRN {
            immediate: u6::new(x),
        },
        "00111000" => Instruction::SSF,
        "00111100" => Instruction::RSF,
        _ => Instruction::NOP,
    }
}
