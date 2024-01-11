use std::iter::Peekable;
use std::num::ParseIntError;
use crate::lexer::{InstructionKind, NumberLiteralKind, Register, Token, TokenKind};

pub enum Node<'a> {
    Label { name: &'a str },
    LabelReference { name: &'a str },
    Instruction { kind: InstructionKind, arguments: Vec<Node<'a>> },
    RegisterLiteral { register: Register },
    NumberLiteral { value: u8 },
}

#[derive(Eq, PartialEq)]
pub enum ErrorTokenKind {
    Newline,
    Colon,
    LabelIdentifier,
    Instruction,
    NumberLiteral,
    RegisterLiteral,
}

pub enum ParseErrorKind {
    InvalidNumberLiteral { cause: ParseIntError },
    UnexpectedToken { expected_types: Vec<ErrorTokenKind> },
}

pub struct ParseError<'a> {
    token: Option<Token<'a>>,
    kind: ParseErrorKind,
    help: Option<String>,
}

// TODO: clean up in general
pub struct Parser<'a, TIter> where TIter: Iterator<Item=Token<'a>> {
    input_tokens: Peekable<TIter>,
}

impl<'a, TIter> Parser<'a, TIter> where TIter: Iterator<Item=Token<'a>> {
    pub fn new(tokens: TIter) -> Self {
        Parser {
            input_tokens: tokens.peekable()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, Vec<ParseError>> {
        let mut program: Vec<Node> = vec![];
        let mut errors: Vec<ParseError> = vec![];

        while let Some(token) = self.input_tokens.next() {
            match token {
                Token { kind: TokenKind::Newline, .. } => {}
                Token { kind: TokenKind::LabelIdentifier, text, .. } => {
                    match self.parse_label(text) {
                        Ok(n) => program.push(n),
                        Err(err) => errors.push(err)
                    }
                }
                Token { kind: TokenKind::Instruction { kind }, .. } => {
                    match self.parse_instruction(kind) {
                        Ok(n) => program.push(n),
                        Err(err) => errors.push(err)
                    }
                }
                other => errors.push(ParseError {
                    token: Some(other),
                    kind: ParseErrorKind::UnexpectedToken {
                        expected_types: vec![
                            ErrorTokenKind::Newline, ErrorTokenKind::LabelIdentifier, ErrorTokenKind::Instruction,
                        ]
                    },
                    help: None,
                })
            }
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }

    pub fn parse_label(&mut self, text: &'a str) -> Result<Node<'a>, ParseError<'a>> {
        match self.input_tokens.next() {
            Some(Token { kind: TokenKind::Colon, .. }) => Ok(Node::Label { name: text }),
            other => Err(ParseError {
                token: other,
                kind: ParseErrorKind::UnexpectedToken { expected_types: vec![ErrorTokenKind::Colon] },
                help: None,
            })
        }
    }

    pub fn parse_instruction(&mut self, kind: InstructionKind) -> Result<Node<'a>, ParseError<'a>> {
        let mut args = vec![];

        while let Some(token) = self.input_tokens.next() {
            match token {
                Token { kind: TokenKind::Newline, .. } => break,
                Token { kind: TokenKind::LabelIdentifier, text, .. } =>
                    args.push(Node::LabelReference { name: text }),
                Token { kind: TokenKind::RegisterLiteral { register }, .. } =>
                    args.push(Node::RegisterLiteral { register }),
                Token { kind: TokenKind::NumberLiteral { kind: num_kind }, text, .. } => {
                    match parse_number_literal(num_kind, text) {
                        Ok(value) => args.push(Node::NumberLiteral { value }),
                        Err(err) => return Err(ParseError {
                            token: Some(token),
                            kind: ParseErrorKind::InvalidNumberLiteral { cause: err },
                            help: None,
                        })
                    }
                }
                other => return Err(ParseError {
                    token: Some(other),
                    kind: ParseErrorKind::UnexpectedToken {
                        expected_types: vec![
                            ErrorTokenKind::Newline,
                            ErrorTokenKind::LabelIdentifier,
                            ErrorTokenKind::RegisterLiteral,
                            ErrorTokenKind::NumberLiteral,
                        ]
                    },
                    help: None,
                })
            }
        }

        Ok(Node::Instruction {
            kind,
            arguments: args,
        })
    }
}

fn parse_number_literal(kind: NumberLiteralKind, text: &str) -> Result<u8, ParseIntError> {
    match kind {
        NumberLiteralKind::Decimal => u8::from_str_radix(text, 10),
        NumberLiteralKind::Hex => u8::from_str_radix(&text[2..], 16),
        NumberLiteralKind::Binary => u8::from_str_radix(&text[2..], 2)
    }
}

