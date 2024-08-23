use crate::{
    error::*,
    lexer::token::*,
};

pub mod ast;
use ast::*;

mod scanner;
use scanner::*;

pub struct Parser {
    errors: Vec<Error>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: &[Token]) {
        let mut scanner = Scanner::new(&tokens);
        
        while !scanner.is_at_end() {
            let symbol = self.parse_symbol(&mut scanner);
        }
    }

    fn parse_symbol(&mut self, scanner: &mut Scanner) -> Symbol {
        
        todo!()
    }
}
