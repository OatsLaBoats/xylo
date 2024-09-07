pub mod token;
use token::*;

mod scanner;
use scanner::*;

mod utils;
use utils::*;

use crate::error::*;

// TODO: Handle invalid number literals like 1012398#%X
// TODO: Deal with numbers that are in the wrong base.
// TODO: Add strings

pub fn lex(input: &[u8]) -> Result<Vec<Token>, Vec<Error>> {
    let mut scanner = Scanner {
        chars: input,
        current_pos: 0,
    };

    let mut result: Vec<Token> = Vec::new();

    loop {
        scanner.skip_whitespace();
        if scanner.is_at_end() { break }

        if scanner.peek().is_ascii() {
            let token = scan_token(&mut scanner);
            result.push(token);
        } else {
            // TODO: Handle invalid ascii character errors
            todo!()
        }
    }

    return Ok(result);
}

fn scan_token(scanner: &mut Scanner) -> Token {
    let start_pos = scanner.current_pos;
    let c = scanner.advance();

    match c {
        '(' => Token { kind: TokenKind::LParen, lexeme: scanner.get_lexeme(start_pos) },
        ')' => Token { kind: TokenKind::RParen, lexeme: scanner.get_lexeme(start_pos) },
        '0'..'9' => scan_number_literal(scanner),
        _ => scan_identifier(scanner),
    }
}

fn scan_identifier(scanner: &mut Scanner) -> Token {
    let start_pos = scanner.backtrack();

    while !scanner.is_at_end() {
        let c = scanner.peek();
        if c.is_whitespace() || is_special_char(c) { break }
        scanner.advance();
    }

    return Token { kind: TokenKind::Identifier, lexeme: scanner.get_lexeme(start_pos) };
}

fn scan_number_literal(scanner: &mut Scanner) -> Token {
    let mut start_pos = scanner.backtrack();

    let kind = if scanner.check_s("0x") {
        scanner.advance_n(2);
        start_pos = scanner.current_pos;

        while !scanner.is_at_end() && scanner.peek().is_ascii_hexdigit() {
            scanner.advance();
        }

        TokenKind::Number(NumberKind::Base16)
    } else if scanner.check_s("0o") {
        scanner.advance_n(2);
        start_pos = scanner.current_pos;
        let range = '0'..'8';

        while !scanner.is_at_end() && range.contains(&scanner.peek()) {
            scanner.advance();
        }

        TokenKind::Number(NumberKind::Base8)
    } else if scanner.check_s("0b") {
        scanner.advance_n(2);
        start_pos = scanner.current_pos;
        let range = '0'..'2';

        while !scanner.is_at_end() && range.contains(&scanner.peek()) {
            scanner.advance();
        }

        TokenKind::Number(NumberKind::Base2)
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

        TokenKind::Number(if is_float { NumberKind::Decimal } else { NumberKind::Base10 })
    };

    return Token {
        kind,
        lexeme: scanner.get_lexeme(start_pos),
    };
}
