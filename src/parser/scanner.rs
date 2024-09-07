use crate::{
    lexer::token::*,
    error::*,
};

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

    pub fn backtrack(&mut self) {
        self.current_pos -= 1;
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current_pos]
    }

    pub fn is_at_end(&self) -> bool {
        self.current_pos >= self.tokens.len()
    }

    pub fn expect(&mut self, expected_token: TokenKind, error_message: String) -> Result<&Token, Error> {
        if !self.matches(expected_token) {
            if !self.is_at_end() { self.advance(); }
            
            Err(Error{
                kind: ErrorKind::SyntaxError,
                line: 0,
                column: 0,
                file: String::from(""),
                message: error_message,
            })
        } else {
            Ok(self.advance())
        }
    }

    pub fn matches(&self, expected_token: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == expected_token
    }
}
