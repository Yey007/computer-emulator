use crate::location::Location;

pub enum NumberLiteralKind {
    Decimal,
    Hex,
    Binary,
}

pub enum TokenKind {
    Newline,
    Colon,
    Comma,
    // Includes instructions, labels, and non-literal instruction parameters
    Identifier,
    NumberLiteral { kind: NumberLiteralKind },
}

pub struct Token {
    kind: TokenKind,
    location: Location,
    text: String,
}

pub struct Lexer {
    program: String,
    location: Location,
    tokens: Vec<Token>,
    buffer_start_location: Location,
    buffer: String,
}

impl Lexer {
    pub fn new(program: String) -> Lexer {
        Lexer {
            program,
            location: Location::start(),
            tokens: vec![],
            buffer_start_location: Location::start(),
            buffer: "".to_owned(),
        }
    }

    pub fn lex(&mut self) {
        loop {
            // TODO: and this is also annoying. We should take "make invalid states unrepresentable" seriously?
            if let Some(current) = self.current_char() {
                match current {
                    '\n' => self.push_buffer_and_current(current, TokenKind::Newline),
                    c if c.is_whitespace() => self.push_buffer(),
                    ':' => self.push_buffer_and_current(current, TokenKind::Colon),
                    ',' => self.push_buffer_and_current(current, TokenKind::Comma),
                    ';' => self.handle_comment(),
                    c => {
                        // TODO: this is hacky
                        if self.buffer.is_empty() {
                            self.buffer_start_location = self.location
                        }
                        self.buffer.push(c)
                    }
                };
            } else {
                self.push_buffer();
                break;
            }

            // TODO: this is annoying too
            if !self.advance() {
                self.push_buffer();
                break;
            }
        }
    }

    fn push_buffer_and_current(&mut self, current: char, kind: TokenKind) {
        self.push_buffer();
        self.push_current(current, kind);
    }

    fn push_current(&mut self, current: char, kind: TokenKind) {
        self.tokens.push(
            Token { kind, location: self.location, text: current.to_string() }
        )
    }

    fn handle_comment(&mut self) {
        while self.advance() {
            // TODO: CRLF
            if let Some('\n') = self.current_char() {
                return;
            }
        }
    }

    // TODO: Should we use the buffer for all of these cases?
    fn push_buffer(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        if self.buffer.starts_with(|c: char| c.is_ascii_digit()) {
            let kind = match self.buffer.get(0..2) {
                Some("0x") => NumberLiteralKind::Hex,
                Some("0b") => NumberLiteralKind::Binary,
                _ => NumberLiteralKind::Decimal
            };

            // TODO: clone avoidable here?
            let token = Token {
                kind: TokenKind::NumberLiteral { kind },
                location: self.buffer_start_location,
                text: self.buffer.clone(),
            };

            self.tokens.push(token)
        } else {
            let token = Token {
                kind: TokenKind::Identifier,
                location: self.buffer_start_location,
                text: self.buffer.clone(),
            };

            self.tokens.push(token)
        }

        self.buffer.clear();
    }

    fn advance(&mut self) -> bool {
        let current = self.current_char();

        match current {
            Some('\n') =>
                {
                    self.location.advance_line();
                    true
                }
            Some(_) =>
                {
                    self.location.advance_col();
                    true
                }
            None => false,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.program.as_bytes().get(self.location.index).map(|&c| c as char)
    }
}