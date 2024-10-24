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

// TODO: Maybe I should make some of these functions return an option instead of crashing
// TODO: Make some function that allow you to check if an s expression is a function
impl Token {
    pub fn sexpr(&self) -> Option<&[Token]> {
        match &self.kind {
            TokenKind::SExpr(sexp) => Some(sexp),
            _ => None,
        }
    }

    pub fn sexpr_mut(&mut self) -> Option<&mut Vec<Token>> {
        match &mut self.kind {
            TokenKind::SExpr(sexp) => Some(sexp),
            _ => None,
        }
    }

    pub fn is_sexpr(&self) -> bool {
        match &self.kind {
            TokenKind::SExpr(_) => true,
            _ => false,
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

    pub fn match_first_identifier(&self, s: &str) -> bool {
        self.match_n_identifier(s, 0)
    }

    pub fn match_n_identifier(&self, s: &str, index: usize) -> bool {
        if let TokenKind::SExpr(sexp) = &self.kind {
            if let Some(token) = sexp.get(index) {
                if let TokenKind::Identifier(id) = &token.kind {
                    return id == s;
                }
            }
        }

        return false;
    }

    pub fn identifier(&self) -> Option<&String> {
        match &self.kind {
            TokenKind::Identifier(id) => Some(id),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        fn token_to_string(token: &Token, acc: &mut String, indentation: i32) {
            match &token.kind {
                TokenKind::Identifier(v) => acc.push_str(&format!("Identifier({}) ", v)),
                TokenKind::String(v) => acc.push_str(&format!("String(\"{}\") ", unsafe { String::from_utf8_unchecked(v.to_vec()) })),
                TokenKind::Int(v) => acc.push_str(&format!("Int({}) ", v)),
                TokenKind::UInt(v) => acc.push_str(&format!("UInt({}) ", v)),
                TokenKind::Float(v) => acc.push_str(&format!("Float({}) ", v)),
                TokenKind::SExpr(v) => {
                    if indentation > 0 {
                        acc.push('\n');
                    }
                    
                    for _ in 0..indentation {
                        acc.push_str("  ");
                    }

                    acc.push_str("SExpr( ");
                    
                    for t in v {
                        token_to_string(t, acc, indentation + 1);
                    }

                    acc.push(')');
                },

                TokenKind::TypeExpr(v) => {
                    match v {
                        Type::Unknown => acc.push_str("TypeExpr(Unknown) "),
                        _ => {},
                    }
                }

                _ => {},
            }
        }

        let mut result = String::new();
        token_to_string(self, &mut result, 0);
        return result;
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
