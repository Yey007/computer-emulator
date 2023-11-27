use bitmatch::bitmatch;
use crate::computer::{REGISTER_INDEX_BITS, WORKING_BITS, PORT_INDEX_BITS, PIN_INDEX_BITS, PC_BITS, INSTRUCTION_BITS};
use crate::un::U;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
    STR {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    LOD {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    LDI {
        register_id: U<REGISTER_INDEX_BITS>,
        immediate: U<WORKING_BITS>,
    },
    INC {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    DEC {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    MOV {
        register_from_id: U<REGISTER_INDEX_BITS>,
        register_to_id: U<REGISTER_INDEX_BITS>,
    },
    INP {
        port_id: U<PORT_INDEX_BITS>,
    },
    OUT {
        port_id: U<PORT_INDEX_BITS>,
    },
    SEP {
        pin_id: U<PIN_INDEX_BITS>,
    },
    RSP {
        pin_id: U<PIN_INDEX_BITS>,
    },
    ADD {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    SUB {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    BOR {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    AND {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    CMP {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    GRT {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    LES {
        register_id: U<REGISTER_INDEX_BITS>,
    },
    BRN {
        immediate: U<PC_BITS>,
    },
    SSF,
    RSF,
}

#[bitmatch]
pub fn decode_instruction(binary: U<INSTRUCTION_BITS>) -> Instruction {
    let a: u8 = binary.into();

    #[bitmatch]
    match a {
        "000001rr" => Instruction::STR {
            register_id: r.into(),
        },
        "000010rr" => Instruction::LOD {
            register_id: r.into(),
        },
        "11rrxxxx" => Instruction::LDI {
            register_id: r.into(),
            immediate: x.into(),
        },
        "000011rr" => Instruction::INC {
            register_id: r.into(),
        },
        "000100rr" => Instruction::DEC {
            register_id: r.into(),
        },
        "0111rrkk" => Instruction::MOV {
            register_from_id: r.into(),
            register_to_id: k.into(),
        },
        "010000pp" => Instruction::INP {
            port_id: p.into(),
        },
        "010001pp" => Instruction::OUT {
            port_id: p.into(),
        },
        "01001qqq" => Instruction::SEP {
            pin_id: q.into(),
        },
        "01010qqq" => Instruction::RSP {
            pin_id: q.into(),
        },
        "000101rr" => Instruction::ADD {
            register_id: r.into(),
        },
        "000110rr" => Instruction::SUB {
            register_id: r.into(),
        },
        "001000rr" => Instruction::BOR {
            register_id: r.into(),
        },
        "001001rr" => Instruction::AND {
            register_id: r.into(),
        },
        "001011rr" => Instruction::CMP {
            register_id: r.into(),
        },
        "001100rr" => Instruction::GRT {
            register_id: r.into(),
        },
        "001101rr" => Instruction::LES {
            register_id: r.into(),
        },
        "10xxxxxx" => Instruction::BRN {
            immediate: x.into(),
        },
        "00111000" => Instruction::SSF,
        "00111100" => Instruction::RSF,
        _ => Instruction::NOP,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_operands() {
        assert_eq!(decode_instruction(0b00111000u8.into()), Instruction::SSF)
    }

    #[test]
    fn register_operand() {
        assert_eq!(
            decode_instruction(0b00000111u8.into()),
            Instruction::STR {
                register_id: 0b11u8.into()
            }
        )
    }

    #[test]
    fn two_register_operands() {
        assert_eq!(
            decode_instruction(0b1110110u8.into()),
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
