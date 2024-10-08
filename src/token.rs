use crate::utils::SourceInfo;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub si: SourceInfo,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    SExpr(Vec<Token>),
    SweetExpr(Vec<Token>),

    TypeExpr(Type),

    Identifier(String),
    
    // Untyped integer literals
    Int(i128),
    UInt(u128),

    // Untyped float literal
    Float(f64),

    // String literal
    String(Vec<u8>),
}

impl Token {
    pub fn get_sexp(&self) -> &[Token] {
        match &self.kind {
            TokenKind::SExpr(sexp) => sexp,
            _ => unreachable!(),
        }
    }

    pub fn get_sexp_mut(&mut self) -> &mut Vec<Token> {
        match &mut self.kind {
            TokenKind::SExpr(sexp) => sexp,
            _ => unreachable!(),
        }
    }

    pub fn is_identifier(&self) -> bool {
        match &self.kind {
            TokenKind::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is_type(&self) -> bool {
        match &self.kind {
            TokenKind::TypeExpr(_) => true,
            _ => false,
        }
    }

    pub fn match_identifier(&self, s: &str) -> bool {
        match &self.kind {
            TokenKind::Identifier(name) => name == s,
            _ => false,
        }
    }

    pub fn get_identifier(&self) -> &String {
        match &self.kind {
            TokenKind::Identifier(id) => id,
            _ => unreachable!(),
        }
    }
}

// [Int]
// [Int, Int -> int]
// [List Int]
// [Either Int Bool]
// [List a]
// [Either a b]
// [a, b -> c]
// [Num a, Eq a b => a, b -> c]

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,

    Simple(String),

    Generic {
        name: String,
        traits: Vec<String>,
    },

    Complex {
        name: String,
        params: Vec<Type>,
    },

    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}
