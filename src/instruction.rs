use crate::types::{InstructionBitType, PinIndex, PortIndex, ProgramCounterType, RegisterIndex, WorkingType};
use bitmatch::bitmatch;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
    STR {
        register_id: RegisterIndex,
    },
    LOD {
        register_id: RegisterIndex,
    },
    LDI {
        register_id: RegisterIndex,
        immediate: WorkingType,
    },
    INC {
        register_id: RegisterIndex,
    },
    DEC {
        register_id: RegisterIndex,
    },
    MOV {
        register_from_id: RegisterIndex,
        register_to_id: RegisterIndex,
    },
    INP {
        port_id: PortIndex,
    },
    OUT {
        port_id: PortIndex,
    },
    SEP {
        pin_id: PinIndex,
    },
    RSP {
        pin_id: PinIndex,
    },
    ADD {
        register_id: RegisterIndex,
    },
    SUB {
        register_id: RegisterIndex,
    },
    BOR {
        register_id: RegisterIndex,
    },
    AND {
        register_id: RegisterIndex,
    },
    CMP {
        register_id: RegisterIndex,
    },
    GRT {
        register_id: RegisterIndex,
    },
    LES {
        register_id: RegisterIndex,
    },
    BRN {
        immediate: ProgramCounterType,
    },
    SSF,
    RSF,
}

#[bitmatch]
pub fn decode_instruction(binary: InstructionBitType) -> Instruction {
    #[bitmatch]
    match binary {
        "000001rr" => Instruction::STR {
            register_id: RegisterIndex::new(r),
        },
        "000010rr" => Instruction::LOD {
            register_id: RegisterIndex::new(r),
        },
        "11rrxxxx" => Instruction::LDI {
            register_id: RegisterIndex::new(r),
            immediate: WorkingType::new(x),
        },
        "000011rr" => Instruction::INC {
            register_id: RegisterIndex::new(r),
        },
        "000100rr" => Instruction::DEC {
            register_id: RegisterIndex::new(r),
        },
        "0111rrkk" => Instruction::MOV {
            register_from_id: RegisterIndex::new(r),
            register_to_id: RegisterIndex::new(k),
        },
        "010000pp" => Instruction::INP {
            port_id: RegisterIndex::new(p),
        },
        "010001pp" => Instruction::OUT {
            port_id: RegisterIndex::new(p),
        },
        "01001qqq" => Instruction::SEP {
            pin_id: PinIndex::new(q),
        },
        "01010qqq" => Instruction::RSP {
            pin_id: PinIndex::new(q),
        },
        "000101rr" => Instruction::ADD {
            register_id: RegisterIndex::new(r),
        },
        "000110rr" => Instruction::SUB {
            register_id: RegisterIndex::new(r),
        },
        "001000rr" => Instruction::BOR {
            register_id: RegisterIndex::new(r),
        },
        "001001rr" => Instruction::AND {
            register_id: RegisterIndex::new(r),
        },
        "001011rr" => Instruction::CMP {
            register_id: RegisterIndex::new(r),
        },
        "001100rr" => Instruction::GRT {
            register_id: RegisterIndex::new(r),
        },
        "001101rr" => Instruction::LES {
            register_id: RegisterIndex::new(r),
        },
        "10xxxxxx" => Instruction::BRN {
            immediate: ProgramCounterType::new(x),
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
        assert_eq!(decode_instruction(0b00111000), Instruction::SSF)
    }

    #[test]
    fn register_operand() {
        assert_eq!(
            decode_instruction(0b00000111),
            Instruction::STR {
                register_id: RegisterIndex::new(0b11)
            }
        )
    }

    #[test]
    fn two_register_operands() {
        assert_eq!(
            decode_instruction(0b1110110),
            Instruction::MOV {
                register_from_id: RegisterIndex::new(0b01),
                register_to_id: RegisterIndex::new(0b10)
            }
        )
    }

    #[test]
    fn register_and_immediate_operands() {
        assert_eq!(
            decode_instruction(0b11001100),
            Instruction::LDI {
                register_id: RegisterIndex::new(0b00),
                immediate: WorkingType::new(0b1100)
            }
        )
    }
}
