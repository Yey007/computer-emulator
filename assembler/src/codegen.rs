use bitmatch::bitmatch;
use common::instruction::Instruction;



// TODO: This code is bad. I'll write a proc macro later, and we can do decode too.
#[bitmatch]
fn encode_instruction(inst: Instruction) -> u8 {
    match inst {
        Instruction::NOP => 0,
        Instruction::STR { register_id } => {
            let r = register_id.into();
            bitpack!("001000rr")
        }
        Instruction::LOD { register_id } => {
            let r = register_id.into();
            bitpack!("001001rr")
        }
        Instruction::LDI { register_id, immediate } => {
            let r = register_id.into();
            let x = immediate.into();
            bitpack!("11rrxxxx")
        }
        Instruction::INC { register_id } => {
            let r = register_id.into();
            bitpack!("001010rr")
        }
        Instruction::DEC { register_id } => {
            let r = register_id.into();
            bitpack!("001011rr")
        }
        Instruction::MOV { register_from_id, register_to_id } => {
            let r = register_from_id.into();
            let k = register_to_id.into();
            bitpack!("0100rrkk")
        }
        Instruction::INP { port_id } => {
            let p = port_id.into();
            bitpack!("011100pp")
        }
        Instruction::OUT { port_id } => {
            let p = port_id.into();
            bitpack!("011101pp")
        }
        Instruction::SEP { pin_id } => {
            let q = pin_id.into();
            bitpack!("011111qq")
        }
        Instruction::RSP { pin_id } => {
            let q = pin_id.into();
            bitpack!("011110qq")
        }
        Instruction::ADD { .. } => {}
        Instruction::SUB { .. } => {}
        Instruction::BOR { .. } => {}
        Instruction::AND { .. } => {}
        Instruction::CMP { .. } => {}
        Instruction::GRT { .. } => {}
        Instruction::LES { .. } => {}
        Instruction::BRN { .. } => {}
        Instruction::LPB { .. } => {}
        Instruction::SSJ => {}
        Instruction::RSJ => {}
        Instruction::RET => {}
        Instruction::SSF => {}
        Instruction::RSF => {}
    }
}