use crate::types::{InstructionBitType, ProgramCounterType, RegisterIndexType, WorkingType};
use bitmatch::bitmatch;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
    STR {
        register_id: RegisterIndexType,
    },
    LOD {
        register_id: RegisterIndexType,
    },
    LDI {
        register_id: RegisterIndexType,
        immediate: WorkingType,
    },
    INC {
        register_id: RegisterIndexType,
    },
    DEC {
        register_id: RegisterIndexType,
    },
    MOV {
        register_from_id: RegisterIndexType,
        register_to_id: RegisterIndexType,
    },
    INP {
        port_id: RegisterIndexType,
    },
    OUT {
        port_id: RegisterIndexType,
    },
    ADD {
        register_id: RegisterIndexType,
    },
    SUB {
        register_id: RegisterIndexType,
    },
    BOR {
        register_id: RegisterIndexType,
    },
    AND {
        register_id: RegisterIndexType,
    },
    CMP {
        register_id: RegisterIndexType,
    },
    GRT {
        register_id: RegisterIndexType,
    },
    LES {
        register_id: RegisterIndexType,
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
            register_id: RegisterIndexType::new(r),
        },
        "000010rr" => Instruction::LOD {
            register_id: RegisterIndexType::new(r),
        },
        "11rrxxxx" => Instruction::LDI {
            register_id: RegisterIndexType::new(r),
            immediate: WorkingType::new(x),
        },
        "000011rr" => Instruction::INC {
            register_id: RegisterIndexType::new(r),
        },
        "000100rr" => Instruction::DEC {
            register_id: RegisterIndexType::new(r),
        },
        "0111rrkk" => Instruction::MOV {
            register_from_id: RegisterIndexType::new(r),
            register_to_id: RegisterIndexType::new(k),
        },
        "010000pp" => Instruction::INP {
            port_id: RegisterIndexType::new(p),
        },
        "010001pp" => Instruction::OUT {
            port_id: RegisterIndexType::new(p),
        },
        "000101rr" => Instruction::ADD {
            register_id: RegisterIndexType::new(r),
        },
        "000110rr" => Instruction::SUB {
            register_id: RegisterIndexType::new(r),
        },
        "001000rr" => Instruction::BOR {
            register_id: RegisterIndexType::new(r),
        },
        "001001rr" => Instruction::AND {
            register_id: RegisterIndexType::new(r),
        },
        "001011rr" => Instruction::CMP {
            register_id: RegisterIndexType::new(r),
        },
        "001100rr" => Instruction::GRT {
            register_id: RegisterIndexType::new(r),
        },
        "001101rr" => Instruction::LES {
            register_id: RegisterIndexType::new(r),
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
                register_id: RegisterIndexType::new(0b11)
            }
        )
    }

    #[test]
    fn two_register_operands() {
        assert_eq!(
            decode_instruction(0b1110110),
            Instruction::MOV {
                register_from_id: RegisterIndexType::new(0b01),
                register_to_id: RegisterIndexType::new(0b10)
            }
        )
    }

    #[test]
    fn register_and_immediate_operands() {
        assert_eq!(
            decode_instruction(0b11001100),
            Instruction::LDI {
                register_id: RegisterIndexType::new(0b00),
                immediate: WorkingType::new(0b1100)
            }
        )
    }
}
