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
