use crate::{
    error::*,
    lexer::token::*,
};

mod scanner;
use scanner::*;

#[derive(Debug)]
pub enum Symbol {
    Identifier(String),
    Int(i64),
    Float(f64),
    SExpr(Vec<Symbol>),
}

impl Symbol {
    pub fn get_sexpr(&self) -> Option<&[Symbol]> {
        if let Self::SExpr(sexpr) = self {
            Some(sexpr) 
        } else {
            None
        }
    }

    pub fn match_ident(&self, ident: &str) -> bool {
        if let Self::Identifier(id) = self {
            id == ident
        } else {
            false
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Symbol>, Vec<Error>>  {
    let mut scanner = Scanner::new(tokens);

    let mut sexps = Vec::new();
    let mut errors = Vec::new();

    while !scanner.is_at_end()  {
        match parse_sexp(&mut scanner) {
            Err(error) => errors.push(error),
            Ok(sexp) => sexps.push(sexp),
        }
    }

    return if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(sexps)
    }
}

fn parse_sexp(scanner: &mut Scanner) -> Result<Symbol, Error> {
    let _ = scanner.expect(TokenKind::LParen, String::from("Expected '('"))?;

    let mut result: Vec<Symbol> = Vec::new();

    while !scanner.is_at_end() {
        let t = scanner.advance();
        match t.kind {
            TokenKind::Identifier => result.push(Symbol::Identifier(t.lexeme.clone())),

            TokenKind::Number(kind) => {
                match kind {
                    NumberKind::Decimal => {
                        let f = t.lexeme.parse::<f64>().unwrap();
                        result.push(Symbol::Float(f));
                    },

                    // TODO: Parse these into an u64 instead. Only Base10 can be negative.
                    // TODO: Handle parsing errors either here or in the lexer.

                    NumberKind::Base2 => {
                        let i = i64::from_str_radix(&t.lexeme, 2).unwrap();
                        result.push(Symbol::Int(i));
                    },

                    NumberKind::Base8 => {
                        let i = i64::from_str_radix(&t.lexeme, 8).unwrap();
                        result.push(Symbol::Int(i));
                    },

                    NumberKind::Base10 => {
                        let i = t.lexeme.parse::<i64>().unwrap();
                        result.push(Symbol::Int(i));
                    },

                    NumberKind::Base16 => {
                        let i = i64::from_str_radix(&t.lexeme,  16).unwrap();
                        result.push(Symbol::Int(i));
                    },
                }
            },

            TokenKind::LParen => {
                scanner.backtrack();
                result.push(parse_sexp(scanner)?);
            },

            TokenKind::RParen => {
                scanner.backtrack();
                break;
            },
        }
    }

    let _ = scanner.expect(TokenKind::RParen, String::from("Expected ')'"))?;

    return Ok(Symbol::SExpr(result));
}
