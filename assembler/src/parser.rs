use std::num::ParseIntError;
use crate::lexer::{NumberLiteralKind, Token, TokenKind};

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

impl InstructionKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nop" => Some(InstructionKind::NOP),
            "str" => Some(InstructionKind::STR),
            "lod" => Some(InstructionKind::LOD),
            "ldi" => Some(InstructionKind::LDI),
            "inc" => Some(InstructionKind::INC),
            "dec" => Some(InstructionKind::DEC),
            "mov" => Some(InstructionKind::MOV),
            "inp" => Some(InstructionKind::INP),
            "out" => Some(InstructionKind::OUT),
            "sep" => Some(InstructionKind::SEP),
            "rsp" => Some(InstructionKind::RSP),
            "add" => Some(InstructionKind::ADD),
            "sub" => Some(InstructionKind::SUB),
            "bor" => Some(InstructionKind::BOR),
            "and" => Some(InstructionKind::AND),
            "cmp" => Some(InstructionKind::CMP),
            "grt" => Some(InstructionKind::GRT),
            "les" => Some(InstructionKind::LES),
            "brn" => Some(InstructionKind::BRN),
            "ssj" => Some(InstructionKind::SSJ),
            "rsj" => Some(InstructionKind::RSJ),
            "ret" => Some(InstructionKind::RET),
            "ssf" => Some(InstructionKind::SSF),
            "rsf" => Some(InstructionKind::RSF),
            _ => None,
        }
    }
}

pub enum Register {
    A,
    X,
    Y,
    Z,
}

pub enum Node<'a> {
    Label { name: &'a str },
    LabelReference { name: &'a str },
    Instruction { kind: InstructionKind, arguments: Vec<Node<'a>> },
    RegisterLiteral { register: Register },
    NumberLiteral { value: u8 },
}

pub enum ParseErrorKind {
    InvalidNumberLiteral { cause: ParseIntError },
    UnknownInstruction,
    UnknownRegister,
    UnexpectedToken { expected_types: Vec<TokenKind> },
}

pub struct ParseError<'a> {
    token: Option<&'a Token<'a>>,
    kind: ParseErrorKind,
    help: Option<String>,
}

// TODO: clean up in general
pub struct Parser<'a, TIter> where TIter: Iterator<Item=&'a Token<'a>> {
    input_tokens: TIter,
}

impl<'a, TIter> Parser<'a, TIter> where TIter: Iterator<Item=&'a Token<'a>> {
    pub fn new(tokens: TIter) -> Self {
        Parser {
            input_tokens: tokens
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut program: Vec<Node> = vec![];

        program.push(self.parse_label()?);

        Ok(program)
    }

    fn parse_label(&mut self) -> Result<Node, ParseError> {
        let first = self.input_tokens.next();
        if let Some(Token { kind: TokenKind::Identifier, text, .. }) = first {
            let second = self.input_tokens.next();
            if let Some(Token { kind: TokenKind::Colon, .. }) = second {
                Ok(Node::Label { name: text })
            } else {
                Err(unexpected_token(second, vec![TokenKind::Colon]))
            }
        } else {
            Err(unexpected_token(first, vec![TokenKind::Identifier]))
        }
    }

    // fn parse_instruction(&mut self) -> Result<Node, ParseError> {
    //     let instruction_kind =
    //         match self.input_tokens.next() {
    //             Some(token @ Token { kind: TokenKind::Identifier, text, .. }) =>
    //                 InstructionKind::from_str(text).ok_or(ParseError {
    //                     token: Some(token),
    //                     kind: UnknownInstruction,
    //                     help: None,
    //                 }),
    //             other => Err(unexpected_token(other, vec![TokenKind::Identifier]))
    //         }?;
    // }

    fn parse_number_literal(&mut self) -> Result<Node, ParseError> {
        let next = self.input_tokens.next();

        if let Some(token @ Token { kind: TokenKind::NumberLiteral { kind }, text, .. }) = next {
            let value = match kind {
                NumberLiteralKind::Decimal => u8::from_str_radix(text, 10),
                NumberLiteralKind::Hex => u8::from_str_radix(&text[2..], 16),
                NumberLiteralKind::Binary => u8::from_str_radix(&text[2..], 2)
            };

            match value {
                Ok(value) => Ok(Node::NumberLiteral { value }),
                Err(err) => Err(ParseError {
                    token: Some(token),
                    kind: ParseErrorKind::InvalidNumberLiteral { cause: err },
                    help: None,
                })
            }
        } else {
            let expected = vec![
                TokenKind::NumberLiteral { kind: NumberLiteralKind::Binary },
                TokenKind::NumberLiteral { kind: NumberLiteralKind::Decimal },
                TokenKind::NumberLiteral { kind: NumberLiteralKind::Hex },
            ];
            Err(unexpected_token(next, expected))
        }
    }

    // fn parse_register_literal(&mut self) -> Result<Node, ParseError> {
    //     let token =
    // }
}

// TODO: help
fn unexpected_token<'a>(maybe_token: Option<&'a Token<'a>>, expected_types: Vec<TokenKind>) -> ParseError {
    ParseError {
        token: maybe_token,
        kind: ParseErrorKind::UnexpectedToken { expected_types },
        help: None,
    }
}

