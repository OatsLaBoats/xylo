pub mod token;
use token::*;

mod scanner;
use scanner::*;

mod utils;
use utils::*;

use crate::error::*;

// TODO: Handle invalid number literals like 1012398#%X
// TODO: Add strings

pub struct Lexer {
    errors: Vec<Error>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&mut self) -> Vec<Error> {
        let result = self.errors.clone();
        self.errors.clear();
        return result;
    }

    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn tokenize(&mut self, input: &[u8]) -> Vec<Token> {
        let mut scanner = Scanner {
            chars: input,
            current_pos: 0,
        };

        let mut result: Vec<Token> = Vec::new();

        loop {
            scanner.skip_whitespace();
            if scanner.is_at_end() { break }

            if scanner.peek().is_ascii() {
                let token = self.scan_token(&mut scanner);
                result.push(token);
            } else {
                // TODO: Handle invalid ascii character errors
                todo!()
            }
        }

        return result;
    }

    fn scan_token(&mut self, scanner: &mut Scanner) -> Token {
        let start_pos = scanner.current_pos;
        let c = scanner.advance();

        match c {
            '(' => Token { kind: TokenKind::LParen, lexeme: scanner.get_lexeme(start_pos) },
            ')' => Token { kind: TokenKind::RParen, lexeme: scanner.get_lexeme(start_pos) },
            '0'..'9' => self.scan_number_literal(scanner),
            _ => self.scan_identifier(scanner),
        }
    }

    fn scan_identifier(&mut self, scanner: &mut Scanner) -> Token {
        let start_pos = scanner.backtrack();

        while !scanner.is_at_end() {
            let c = scanner.peek();
            if c.is_whitespace() || is_special_char(c) { break }
            scanner.advance();
        }

        return Token { kind: TokenKind::Identifier, lexeme: scanner.get_lexeme(start_pos) };
    }

    fn scan_number_literal(&mut self, scanner: &mut Scanner) -> Token {
        let mut start_pos = scanner.backtrack();

        let kind = if scanner.check_s("0x") {
            scanner.advance_n(2);
            start_pos = scanner.current_pos;

            while !scanner.is_at_end() && scanner.peek().is_ascii_hexdigit() {
                scanner.advance();
            }

            TokenKind::Number(NumberKind::Hexadecimal)
        } else if scanner.check_s("0o") {
            scanner.advance_n(2);
            start_pos = scanner.current_pos;
            let range = '0'..'8';

            while !scanner.is_at_end() && range.contains(&scanner.peek()) {
                scanner.advance();
            }

            TokenKind::Number(NumberKind::Octal)
        } else if scanner.check_s("0b") {
            scanner.advance_n(2);
            start_pos = scanner.current_pos;
            let range = '0'..'2';

            while !scanner.is_at_end() && range.contains(&scanner.peek()) {
                scanner.advance();
            }

            TokenKind::Number(NumberKind::Binary)
        } else {
            let mut is_float = false;
            while !scanner.is_at_end() {
                if scanner.check('.') && !is_float {
                    is_float = true;
                } else if !scanner.peek().is_ascii_digit() {
                    break;
                }

                scanner.advance();
            }

            TokenKind::Number(if is_float { NumberKind::Float } else { NumberKind::Decimal })
        };

        return Token {
            kind,
            lexeme: scanner.get_lexeme(start_pos),
        };
    }
}
