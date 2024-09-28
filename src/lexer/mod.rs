use crate::utils::*;
use std::{
    str::FromStr,
    num::IntErrorKind,
};

// TODO: Clean this up
// TODO: Maybe create a function that skips until the next S-Expression in certain cases

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub si: SourceInfo,
}

#[derive(Debug)]
pub enum TokenKind {
    SExpr(Vec<Token>),
    TypeExpr(Vec<Token>),
    SweetExpr(Vec<Token>),

    Identifier(String),
    
    // Untyped integer literals
    Int(i128),
    UInt(u128),

    // Untyped float literal
    Float(f64),
}

// TODO: Might want to skip until the start of the next S-Expression on error.
pub fn lex(ascii_text: &[u8]) -> Result<Vec<Token>, Vec<Error>> {
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
        
        _ => todo!(),
    }
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
        let s = std::str::from_utf8(slice).unwrap(); // This should never fail.

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

struct Scanner<'a> {
    text: &'a [u8],
    index: usize,

    line: i64,
    column: i64,

    newline: bool,
}

impl<'a> Scanner<'a> {
    fn new(text: &'a [u8]) -> Self {
        Self {
            text,
            index: 0,

            line: 1,
            column: 1,

            newline: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.text.len()
    }

    fn advance(&mut self) -> char {
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

    fn skip(&mut self, amount: usize) {
        for _ in 0..amount {
            if self.advance() == '\0' {
                break;
            }
        }
    }

    fn peek(&self) -> char {
        self.text[self.index] as char
    }

    fn match_string(&self, s: &str) -> bool {
        if self.text.len() < s.len() + self.index {
            return false;
        }

        let slice = &self.text[self.index .. self.index + s.len()];
        return slice == s.as_bytes();
    }

    fn match_char(&self, c: char) -> bool {
        self.peek() == c
    }

    fn get_source_info(&self) -> SourceInfo {
        SourceInfo::new(self.line, self.column, self.index)
    }
}
