use crate::lexer::token::*;

pub struct Scanner<'a> {
    pub tokens: &'a[Token],
    pub current_pos: usize,
}

impl Scanner<'_> {
    pub fn new(tokens: &[Token]) -> Scanner {
        Scanner {
            tokens,
            current_pos: 0,
        }
    }

    pub fn advance(&mut self) -> &Token {
        let result = &self.tokens[self.current_pos];
        self.current_pos += 1;
        return result;
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current_pos]
    }

    pub fn is_at_end(&self) -> bool {
        self.current_pos >= self.tokens.len()
    }
}
