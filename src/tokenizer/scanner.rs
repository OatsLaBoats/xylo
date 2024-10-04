use crate::utils::SourceInfo;

pub struct Scanner<'a> {
    pub text: &'a [u8],
    pub index: usize,

    line: i64,
    column: i64,

    newline: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a [u8]) -> Self {
        Self {
            text,
            index: 0,

            line: 1,
            column: 1,

            newline: false,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.text.len()
    }

    pub fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.text[self.index] as char;

        if self.newline {
            self.newline = false;
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        if c == '\n' {
            self.newline = true;
        }

        self.index += 1;

        return c;
    }

    pub fn skip(&mut self, amount: usize) {
        for _ in 0..amount {
            if self.advance() == '\0' {
                break;
            }
        }
    }

    pub fn peek(&self) -> char {
        self.text[self.index] as char
    }

    pub fn match_string(&self, s: &str) -> bool {
        if self.text.len() < s.len() + self.index {
            return false;
        }

        let slice = &self.text[self.index .. self.index + s.len()];
        return slice == s.as_bytes();
    }

    pub fn match_char(&self, c: char) -> bool {
        self.peek() == c
    }

    pub fn get_source_info(&self) -> SourceInfo {
        SourceInfo::new(self.line, self.column, self.index)
    }
}
