use crate::architecture::INSTRUCTION_BITS;
use crate::un::U;
use bitmatch::bitmatch;
use instruction_set_gen::make_instructions;

make_instructions!();

#[cfg(test)]
mod tests {
    use super::*;

    mod decode {
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
}
