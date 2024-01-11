use std::iter::Peekable;
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
    input_tokens: Peekable<TIter>,
}

impl<'a, TIter> Parser<'a, TIter> where TIter: Iterator<Item=&'a Token<'a>> {
    pub fn new(tokens: TIter) -> Self {
        Parser {
            input_tokens: tokens.peekable()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut program: Vec<Node> = vec![];

        while let Some(_) = self.input_tokens.peek() {
            while self.skip_optional_newline() { }

            if let Ok(n) = self.parse_label() {
                program.push(n);
                continue;
            }

            match self.parse_instruction() {
                Ok(n) => {
                    program.push(n);
                    continue
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }

        Ok(program)
    }

    fn parse_label(&mut self) -> Result<Node<'a>, ParseError<'a>> {
        let first = self.input_tokens.next();
        let Some(Token { kind: TokenKind::Identifier, text, .. }) = first else {
            return Err(unexpected_token(first, vec![TokenKind::Identifier]));
        };

        let second = self.input_tokens.next();
        let Some(Token { kind: TokenKind::Colon, .. }) = second else {
            return Err(unexpected_token(second, vec![TokenKind::Colon]));
        };

        Ok(Node::Label { name: text })
    }

    fn parse_instruction(&mut self) -> Result<Node<'a>, ParseError<'a>> {
        let instruction_kind =
            match self.input_tokens.next() {
                Some(token @ Token { kind: TokenKind::Identifier, text, .. }) =>
                    InstructionKind::from_str(text).ok_or(ParseError {
                        token: Some(token),
                        kind: ParseErrorKind::UnknownInstruction,
                        help: None,
                    }),
                other => Err(unexpected_token(other, vec![TokenKind::Identifier]))
            }?;

        let mut args = vec![];

        while let Some(token) = self.input_tokens.next() {
            if token.kind == TokenKind::Newline {
                break;
            }

            let num_result = parse_number_literal(token);
            match num_result {
                Ok(n) => {
                    args.push(n);
                    continue;
                }
                Err(err @ ParseError { kind: ParseErrorKind::InvalidNumberLiteral { .. }, .. }) => { return Err(err); }
                Err(_) => {}
            };

            let reg_result = parse_register_literal(token);
            if let Ok(n) = reg_result {
                args.push(n);
                continue;
            }

            let label_result = parse_label_reference(token);
            match label_result {
                Ok(n) => {
                    args.push(n);
                    continue;
                }
                Err(_) => {
                    return Err(unexpected_token(Some(token), vec![
                        TokenKind::NumberLiteral { kind: NumberLiteralKind::Binary },
                        TokenKind::NumberLiteral { kind: NumberLiteralKind::Decimal },
                        TokenKind::NumberLiteral { kind: NumberLiteralKind::Hex },
                        TokenKind::Identifier
                    ]))
                }
            }
        }

        Ok(Node::Instruction { kind: instruction_kind, arguments: args })
    }

    fn skip_optional_newline(&mut self) -> bool {
        if let Some(Token { kind: TokenKind::Newline, .. }) = self.input_tokens.peek() {
            self.input_tokens.next();
            return true
        }
        false
    }
}

fn parse_number_literal<'a>(next: &'a Token) -> Result<Node<'a>, ParseError<'a>> {
    let token @ Token { kind: TokenKind::NumberLiteral { kind }, text, .. } = next else {
        let expected = vec![
            TokenKind::NumberLiteral { kind: NumberLiteralKind::Binary },
            TokenKind::NumberLiteral { kind: NumberLiteralKind::Decimal },
            TokenKind::NumberLiteral { kind: NumberLiteralKind::Hex },
        ];
        return Err(unexpected_token(Some(next), expected));
    };

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
}

fn parse_register_literal<'a>(next: &'a Token) -> Result<Node<'a>, ParseError<'a>> {
    let Token { kind: TokenKind::Identifier, text, .. } = next else {
        return Err(unexpected_token(Some(next), vec![TokenKind::Identifier]));
    };

    let reg = match *text {
        "a" => Ok(Register::A),
        "x" => Ok(Register::X),
        "y" => Ok(Register::Y),
        "z" => Ok(Register::Z),
        _ => Err(ParseError {
            token: Some(next),
            kind: ParseErrorKind::UnknownRegister,
            help: None,
        })
    }?;

    Ok(Node::RegisterLiteral { register: reg })
}

fn parse_label_reference<'a>(next: &'a Token) -> Result<Node<'a>, ParseError<'a>> {
    let Token { kind: TokenKind::Identifier, text, .. } = next else {
        return Err(unexpected_token(Some(next), vec![TokenKind::Identifier]));
    };

    Ok(Node::LabelReference {
        name: text,
    })
}

// TODO: help
fn unexpected_token<'a>(maybe_token: Option<&'a Token<'a>>, expected_types: Vec<TokenKind>) -> ParseError {
    ParseError {
        token: maybe_token,
        kind: ParseErrorKind::UnexpectedToken { expected_types },
        help: None,
    }
}

