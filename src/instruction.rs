use bitmatch::bitmatch;
use ux::{u2, u4, u6};

#[derive(Debug)]
pub enum Instruction {
    NOP,
    STR { register_id: u2 },
    LOD { register_id: u2 },
    LDI { register_id: u2, immediate: u4 },
    INC { register_id: u2 },
    DEC { register_id: u2 },
    INP { register_id: u2, port_id: u2 },
    OUT { register_id: u2, port_id: u2 },
    ADD { register_id: u2 },
    SUB { register_id: u2 },
    BOR { register_id: u2 },
    AND { register_id: u2 },
    CMP { register_id: u2 },
    GRT { register_id: u2 },
    LES { register_id: u2 },
    BRN { immediate: u6 },
    SSF,
    RSF,
}

#[bitmatch]
pub fn decode_instruction(binary: u8) -> Instruction {
    #[bitmatch]
    match binary {
        "000001rr" => Instruction::STR {
            register_id: u2::new(r),
        },
        "000010rr" => Instruction::LOD {
            register_id: u2::new(r),
        },
        "11rrxxxx" => Instruction::LDI {
            register_id: u2::new(r),
            immediate: u4::new(x),
        },
        "000011rr" => Instruction::INC {
            register_id: u2::new(r),
        },
        "000100rr" => Instruction::DEC {
            register_id: u2::new(r),
        },
        "0100rrpp" => Instruction::INP {
            register_id: u2::new(r),
            port_id: u2::new(p),
        },
        "0101rrpp" => Instruction::OUT {
            register_id: u2::new(r),
            port_id: u2::new(p),
        },
        "000101rr" => Instruction::ADD {
            register_id: u2::new(r),
        },
        "000110rr" => Instruction::SUB {
            register_id: u2::new(r),
        },
        "001000rr" => Instruction::BOR {
            register_id: u2::new(r),
        },
        "001001rr" => Instruction::AND {
            register_id: u2::new(r),
        },
        "001011rr" => Instruction::CMP {
            register_id: u2::new(r),
        },
        "001100rr" => Instruction::GRT {
            register_id: u2::new(r),
        },
        "001101rr" => Instruction::LES {
            register_id: u2::new(r),
        },
        "10xxxxxx" => Instruction::BRN {
            immediate: u6::new(x),
        },
        "00111000" => Instruction::SSF,
        "00111100" => Instruction::RSF,
        _ => Instruction::NOP,
    }
}
