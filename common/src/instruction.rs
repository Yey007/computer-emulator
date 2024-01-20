use crate::architecture::*;
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
    LPB {
        immediate: U<PA_BITS>
    },
    SSJ,
    RSJ,
    RET,
    SSF,
    RSF,
}
