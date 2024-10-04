use crate::{
    utils::*,
    token::*,
};

use std::{
    str::FromStr,
    num::IntErrorKind,
};

mod scanner;
use scanner::Scanner;

// TODO: Clean this up
// TODO: Maybe create a function that skips until the next S-Expression in certain cases

// TODO: Might want to skip until the start of the next S-Expression on error.
pub fn tokenize(ascii_text: &[u8]) -> Result<Vec<Token>, Vec<Error>> {
    let mut scanner = Scanner::new(ascii_text);
    let mut errors: Vec<Error> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    skip_whitespace(&mut scanner);
    while !scanner.is_at_end() {
        match scan_sexpr(&mut scanner) {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error),
        }

        skip_whitespace(&mut scanner);
    }

    return if errors.len() > 0 { Err(errors) } else { Ok(tokens) };
}

fn scan_sexpr(scanner: &mut Scanner) -> Result<Token, Error> {
    let sexpr_si = scanner.get_source_info();

    let c = scanner.advance();
    if c != '{' {
        return Err(Error::new(
            "Expected '{' to start an s-expression".to_string(),
            scanner.get_source_info(),
        ));
    }

    let mut sexpr = Vec::new();

    skip_whitespace(scanner);
    while !scanner.is_at_end() && !scanner.match_char('}') {
        let token = scan_token(scanner)?;
        sexpr.push(token);
        skip_whitespace(scanner);
    }

    let c = scanner.advance();
    if c != '}' {
        return Err(Error::new(
            "Expected '}' to end s-expression".to_string(),
            scanner.get_source_info(),
        ));
    }

    return Ok(Token {
        kind: TokenKind::SExpr(sexpr),
        si: sexpr_si,
    });
}

fn scan_token(scanner: &mut Scanner) -> Result<Token, Error> {
    let c = scanner.peek();

    match c {
        '{' => return scan_sexpr(scanner),

        'a'..='z'|'A'..='Z'|
        '!'|'$'..='&'|'*'|'+'|
        '-'|'/'|':'..='@'|'\\'|
        '^'..='`'|'|'|'~' => return scan_identifier(scanner),

        '0'..='9' => return scan_number(scanner),

        '"' => return scan_string(scanner),
        
        _ => todo!(),
    }
}

fn scan_string(scanner: &mut Scanner) -> Result<Token, Error> {
    let si = scanner.get_source_info();
    
    scanner.advance();
    let start = scanner.index;

    while !scanner.is_at_end() {
        let c = scanner.advance();
        if c == '"' {
            let slice = &scanner.text[start..scanner.index - 1];
            return Ok(Token {
                kind: TokenKind::String(slice.to_vec()),
                si,
            });
        }
    }

    return Err(Error {
        message: "Unterminated string literal".to_string(),
        si: scanner.get_source_info(),
    });
}

fn scan_number(scanner: &mut Scanner) -> Result<Token, Error> {
    let si = scanner.get_source_info();
    let mut integer_base = 10;

    if scanner.match_string("0x") {
        integer_base = 16;
        scanner.skip(2);
        todo!()
    } else if scanner.match_string("0o") {
        integer_base = 8;
        scanner.skip(2);
        todo!()
    } else if scanner.match_string("0b") {
        integer_base = 2;
        scanner.skip(2);
        todo!()
    } else {
        let mut is_float = false;
        let mut valid_float = false;
        let start = scanner.index;

        while !scanner.is_at_end() {
            let c = scanner.peek();
            
            if is_float && !valid_float {
                if c.is_ascii_digit() {
                    valid_float = true;
                } else {
                    return Err(Error::new(
                        "Expected at least one digit after '.'".to_string(),
                        scanner.get_source_info(),
                    ));
                }
            }

            if c == '.' {
                if !is_float {
                    is_float = true;
                } else {
                    return Err(Error::new(
                        "Duplicate decimal point in float".to_string(),
                        scanner.get_source_info(),
                    ));
                }
            }

            if !c.is_ascii_digit() && c != '.' {
                break;
            }

            scanner.advance();
        }

        let slice = &scanner.text[start..scanner.index];
        let s = std::str::from_utf8(slice).unwrap();

        if is_float {
            let value = f64::from_str(s).unwrap();
            return Ok(Token {
                kind: TokenKind::Float(value),
                si,
            });
        }

        let value = match u128::from_str_radix(s, integer_base) {
            Ok(v) => v,
            Err(e) => {
                let message = match e.kind() {
                    IntErrorKind::PosOverflow => "Integer literal exeeds maximum integer size",
                    _ => unreachable!(),
                };

                return Err(Error::new(
                    message.to_string(),
                    si,
                )); 
            },
        };

        return Ok(Token {
            kind: TokenKind::UInt(value),
            si,
        });
    }
}

fn scan_identifier(scanner: &mut Scanner) -> Result<Token, Error> {
    let si = scanner.get_source_info();

    while !scanner.is_at_end() {
        let c = scanner.peek();
        if !valid_identifier_char(c) {
            break;
        }

        scanner.advance();
    }

    let slice = &scanner.text[si.index..scanner.index];
    let id = String::from_utf8_lossy(slice);

    return Ok(Token {
        kind: TokenKind::Identifier(id.to_string()),
        si,
    });
}

fn valid_identifier_char(c: char) -> bool {
    c.is_ascii() && (
    c == '!' ||
    (c >= '$' && c <= '&') ||
    c == '*' ||
    c == '+' ||
    c == '-' ||
    (c >= '/' && c <= 'Z') ||
    c == '\\' ||
    (c >= '^' && c <= 'z') ||
    c == '|' ||
    c == '~')
}

// Skips whitespace and comments
fn skip_whitespace(scanner: &mut Scanner) {
    while !scanner.is_at_end() {
        let c = scanner.peek();
        
        if c.is_whitespace() {
            scanner.advance();
        } else if c == '#' {
            scanner.advance();
            if scanner.advance() == '-' {
                while !scanner.is_at_end() {
                    if scanner.advance() == '-' {
                        if scanner.advance() == '#' {
                            break;
                        }
                    }
                }
            } else {
                while !scanner.is_at_end() {
                    if scanner.advance() == '\n' {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
}
