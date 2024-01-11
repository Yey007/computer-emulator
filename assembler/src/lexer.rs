use crate::location::Location;

#[derive(Eq, PartialEq)]
pub enum NumberLiteralKind {
    Decimal,
    Hex,
    Binary,
}

#[derive(Eq, PartialEq)]
pub enum TokenKind {
    Newline,
    Colon,
    // Includes instructions, labels, and non-number instruction parameters
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
            location: Location::start(),
            tokens: vec![],
        }
    }

    pub fn lex(mut self) -> Vec<Token<'a>> {
        while let Some(char) = self.current_char() {
            match char {
                c if let Some(kind) = single_char_token(c) => self.handle_single(kind),
                c if c.is_ascii_digit() => self.handle_number(),
                c if c.is_ascii_whitespace() => self.advance(),
                ';' => self.handle_comment(),
                _ => self.handle_ident()
            };
        }

        self.tokens
    }

    fn handle_single(&mut self, kind: TokenKind) {
        let current = self.current_char_as_str().expect("Current char must be available to handle it");

        self.tokens.push(
            Token { kind, location: self.location, text: current }
        );

        self.advance()
    }

    fn handle_number(&mut self) {
        let start = self.location;
        let num = self.get_sequence();

        let kind = match num.get(0..2) {
            Some("0x") => NumberLiteralKind::Hex,
            Some("0b") => NumberLiteralKind::Binary,
            _ => NumberLiteralKind::Decimal
        };

        self.tokens.push(Token {
            kind: TokenKind::NumberLiteral { kind },
            location: start,
            text: num,
        })
    }

    fn handle_ident(&mut self) {
        let start = self.location;
        let str = self.get_sequence();

        self.tokens.push(Token {
            kind: TokenKind::Identifier,
            location: start,
            text: str,
        })
    }

    fn handle_comment(&mut self) {
        assert!(self.current_char().is_some(), "Current char must be available to handle a comment");

        while let Some(current) = self.current_char() {
            if current == '\n' {
                break;
            }
            self.advance()
        }
    }

    fn get_sequence(&mut self) -> &'a str {
        assert!(self.current_char().is_some(), "Current char must be available to lex a sequence");

        let start = self.location;
        while let Some(current) = self.current_char() {
            if is_sequence_terminator(current) {
                break;
            }
            self.advance()
        }

        &self.program[start.index..self.location.index]
    }

    /// Advances the current location of the lexer, if possible, adjusting `self.location` accordingly.
    fn advance(&mut self) {
        let current = self.current_char();

        if let Some('\n') = current {
            self.location = self.location.advance_line()
        } else if let Some(_) = current {
            self.location = self.location.advance_col()
        }
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
        self.program.get(index..=index)
    }
}

fn is_sequence_terminator(c: char) -> bool {
    single_char_token(c).is_some() || c.is_ascii_whitespace() || c == ';'
}

fn single_char_token(c: char) -> Option<TokenKind> {
    match c {
        '\n' => Some(TokenKind::Newline),
        ':' => Some(TokenKind::Colon),
        _ => None
    }
}
