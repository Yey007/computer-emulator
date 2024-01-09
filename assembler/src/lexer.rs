use std::str::from_utf8;
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

pub struct Token<'a> {
    pub kind: TokenKind,
    pub location: Location,
    pub text: &'a str,
}

pub struct Lexer<'a> {
    /// The program. Contains only ASCII characters.
    program: &'a str,
    /// The start location of the current sequence being processed. Must be <= location.
    sequence_start_location: Location,
    /// A flag indicating that the next advancement should start a new sequence.
    start_new_sequence: bool,
    /// The current location of the lexer in the program.
    location: Location,
    /// The tokens gathered so far.
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer given a program. The program must only contain ASCII characters.
    ///
    /// # Arguments
    ///
    /// * `program`: The program, containing only ASCII characters.
    pub fn new(program: &'a str) -> Self {
        assert!(program.is_ascii());

        Lexer {
            program,
            sequence_start_location: Location::start(),
            start_new_sequence: false,
            location: Location::start(),
            tokens: vec![],
        }
    }

    pub fn lex(mut self) -> Vec<Token<'a>> {
        while let Some(char) = self.current_char() {
            match char {
                '\n' => self.push_sequence_and_current(TokenKind::Newline),
                c if c.is_whitespace() => self.push_sequence(),
                ':' => self.push_sequence_and_current(TokenKind::Colon),
                ',' => self.push_sequence_and_current(TokenKind::Comma),
                ';' => self.handle_comment(),
                _ => {}
            };

            self.advance();
        }

        self.push_sequence();

        self.tokens
    }

    fn push_sequence_and_current(&mut self, kind: TokenKind) {
        self.push_sequence();
        self.push_current(kind);
    }

    fn push_current(&mut self, kind: TokenKind) {
        let Some(current) = self.current_char_as_str() else { return; };

        self.tokens.push(
            Token { kind, location: self.location, text: current }
        )
    }

    fn handle_comment(&mut self) {
        while let Some(current) = self.current_char() {
            if current == '\n' {
                break;
            }
            self.advance()
        }
        self.start_new_sequence = true;
    }

    // TODO: Should we use the sequence for all of these cases?
    fn push_sequence(&mut self) {
        let Some(sequence) = self.current_sequence() else { return; };

        if sequence.is_empty() {
            self.start_new_sequence = true;
            return;
        }

        if sequence.starts_with(|c: char| c.is_ascii_digit()) {
            let kind = match sequence.get(0..2) {
                Some("0x") => NumberLiteralKind::Hex,
                Some("0b") => NumberLiteralKind::Binary,
                _ => NumberLiteralKind::Decimal
            };

            let token = Token {
                kind: TokenKind::NumberLiteral { kind },
                location: self.sequence_start_location,
                text: sequence,
            };

            self.tokens.push(token);
        } else {
            let token = Token {
                kind: TokenKind::Identifier,
                location: self.sequence_start_location,
                text: sequence,
            };

            self.tokens.push(token);
        }

        self.start_new_sequence = true;
    }

    /// Advances the current location of the lexer, if possible, adjusting `self.location` accordingly.
    /// `self.sequence_start_location` will be left untouched.
    fn advance(&mut self) {
        let current = self.current_char();

        // TODO: CRLF
        if let Some('\n') = current {
            self.location.advance_line()
        } else if let Some(_) = current {
            self.location.advance_col()
        }

        if self.start_new_sequence {
            self.sequence_start_location = self.location;
            self.start_new_sequence = false
        }
    }

    /// Returns the current sequence being processed. Will return `None` if the lexer has finished. The returned string
    /// slice may be empty, in which case there is nothing in the sequence, likely due to terminator like a colon or
    /// comma being processed in the previous iteration.
    fn current_sequence(&self) -> Option<&'a str> {
        let start = self.sequence_start_location.index;
        let end = self.location.index;

        if end > self.program.len() {
            return None;
        }

        Some(&self.program[start..end])
    }

    /// Returns the current character if the lexer has not finished. Returns None if it has.
    fn current_char(&self) -> Option<char> {
        self.program.as_bytes().get(self.location.index).map(|&c| c as char)
    }

    // Hack of the century
    /// Does roughly the same thing as current_char, but returns a string slice instead to make
    /// constructing tokens easier.
    fn current_char_as_str(&self) -> Option<&'a str> {
        let index = self.location.index;
        if index >= self.program.len() {
            return None;
        }

        Some(&self.program[index..=index])
    }
}