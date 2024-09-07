use crate::parser::Symbol;
use std::collections::hash_map::HashMap;

// The organizer is a subsystem of the compiler that takes the parsed raw s-expressions and sorts
// them into different categories useful for things like typechecking, static analysis and code
// generation.

// TODO: The organizer is the first part of error checking as it will detect structural errors like
// a function without a name or parameters and such.

pub struct Module {
    pub functions: HashMap<String, Symbol>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            functions: HashMap::new(),
        }
    }

    pub fn add_symbols(&mut self, symbols:  Vec<Symbol>) {
        for s in symbols {
            let sexpr = s.get_sexpr().unwrap();
            let mut found_function = false;

            sexpr
                .first()
                .map(|x| {
                    if x.match_ident("function") {
                        found_function = true;
                    }
                });

            if found_function {
                if let Some(x) = sexpr.get(1) {
                    if let Symbol::Identifier(name) = x {
                        self.functions.insert(name.to_owned(), s);
                    }
                } 
            }
        }
    }
}
