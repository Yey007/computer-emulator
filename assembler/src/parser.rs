use crate::lexer::Token;

pub enum InstructionKind {
    NOP,
    STR,
    LOD,
    LDI,
    INC,
    DEC,
    MOV,
    INP,
    OUT,
    SEP,
    RSP,
    ADD,
    SUB,
    BOR,
    AND,
    CMP,
    GRT,
    LES,
    BRN,
    SSJ,
    RSJ,
    RET,
    SSF,
    RSF,
}

pub enum Register {
    A,
    X,
    Y,
    Z,
}

pub enum Node {
    Label { name: String },
    LabelReference { name: String },
    Instruction { kind: InstructionKind, arguments: Vec<Node> },
    RegisterLiteral { register: Register },
    NumberLiteral { value: u8 }
}

pub struct Parser {
    input_tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            input_tokens: tokens
        }
    }

    pub fn parse(&self) -> Vec<Node> {
        vec![]
    }
}