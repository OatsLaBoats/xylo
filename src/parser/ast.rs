use std::{
    collections::HashMap,
    fmt::Display,
};

pub struct Ast {
    pub functions: HashMap<String, Symbol>,
}

pub enum Symbol {
    Identifier(String),
    Int(i64),
    Float(f64),
    SExpr(Vec<Symbol>),
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            functions: HashMap::new(),
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
