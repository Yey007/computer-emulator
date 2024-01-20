use bitmatch::bitmatch;
use common::instruction::Instruction;
use common::un::U;
use common::architecture::INSTRUCTION_BITS;

#[bitmatch]
pub fn decode_instruction(binary: U<INSTRUCTION_BITS>) -> Instruction {
    let a: u8 = binary.into();

    #[bitmatch]
    match a {
        "001000rr" => Instruction::STR {
            register_id: r.into(),
        },
        "001001rr" => Instruction::LOD {
            register_id: r.into(),
        },
        "11rrxxxx" => Instruction::LDI {
            register_id: r.into(),
            immediate: x.into(),
        },
        "001010rr" => Instruction::INC {
            register_id: r.into(),
        },
        "001011rr" => Instruction::DEC {
            register_id: r.into(),
        },
        "0100rrkk" => Instruction::MOV {
            register_from_id: r.into(),
            register_to_id: k.into(),
        },
        "011100pp" => Instruction::INP {
            port_id: p.into(),
        },
        "011101pp" => Instruction::OUT {
            port_id: p.into(),
        },
        "011111qq" => Instruction::SEP {
            pin_id: q.into(),
        },
        "011110qq" => Instruction::RSP {
            pin_id: q.into(),
        },
        "010100rr" => Instruction::ADD {
            register_id: r.into(),
        },
        "010101rr" => Instruction::SUB {
            register_id: r.into(),
        },
        "010110rr" => Instruction::BOR {
            register_id: r.into(),
        },
        "010111rr" => Instruction::AND {
            register_id: r.into(),
        },
        "011010rr" => Instruction::CMP {
            register_id: r.into(),
        },
        "011000rr" => Instruction::GRT {
            register_id: r.into(),
        },
        "011001rr" => Instruction::LES {
            register_id: r.into(),
        },
        "10xxxxxx" => Instruction::BRN {
            immediate: x.into(),
        },
        "0001xxxx" => Instruction::LPB {
            immediate: x.into(),
        },
        "00000011" => Instruction::SSF,
        "00000010" => Instruction::RSF,
        "00000001" => Instruction::SSJ,
        "00000111" => Instruction::RSJ,
        "00001000" => Instruction::RET,
        _ => Instruction::NOP,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_operands() {
        assert_eq!(decode_instruction(0b00000011u8.into()), Instruction::SSF)
    }

    #[test]
    fn register_operand() {
        assert_eq!(
            decode_instruction(0b00100011u8.into()),
            Instruction::STR {
                register_id: 0b11u8.into()
            }
        )
    }

    #[test]
    fn two_register_operands() {
        assert_eq!(
            decode_instruction(0b01000110u8.into()),
            Instruction::MOV {
                register_from_id: 0b01u8.into(),
                register_to_id: 0b10u8.into()
            }
        )
    }

    #[test]
    fn register_and_immediate_operands() {
        assert_eq!(
            decode_instruction(0b11001100u8.into()),
            Instruction::LDI {
                register_id: 0b00u8.into(),
                immediate: 0b1100u8.into()
            }
        )
    }
}