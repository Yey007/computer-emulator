#[derive(Copy, Clone)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn start() -> Location {
        Location {
            index: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn advance_col(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    pub fn advance_line(&mut self) {
        self.index += 1;
        self.line += 1;
        self.col = 1;
    }
}
