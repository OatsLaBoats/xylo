use std::{
    collections::HashMap, 
    ops::Range,
};

use crate::token::*;

#[derive(Debug)]
pub struct Program {
    modules: Vec<Module>,
    module_lookup: HashMap<String, usize>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            module_lookup: HashMap::new(),
        }
    }

    pub fn get_ids(&self) -> Range<usize> {
        0..self.modules.len()
    }

    pub fn new_module(&mut self, name: String, code: Vec<Token>) -> usize {
        let id = self.modules.len();

        self.module_lookup.insert(name.clone(), id);
        self.modules.push(Module::new(id, name, code));

        return id;
    }

    pub fn get_modules(&self) -> &[Module] {
        &self.modules
    }

    pub fn get_modules_mut(&mut self) -> &mut [Module] {
        &mut self.modules
    }

    pub fn get_module_id(&self, name: &String) -> Option<usize> {
        Some(self.module_lookup.get(name)?.clone())
    }

    pub fn get_module_by_name(&self, name: &String) -> Option<&Module> {
        self.modules.get(self.module_lookup.get(name)?.to_owned())
    }

    pub fn get_module_by_name_mut(&mut self, name: &String) -> Option<&mut Module> {
        self.modules.get_mut(self.module_lookup.get(name)?.to_owned())
    }

    pub fn get_module_by_id(&self, id: usize) -> Option<&Module> {
        self.modules.get(id)
    }

    pub fn get_module_by_id_mut(&mut self, id: usize) -> Option<&mut Module> {
        self.modules.get_mut(id)
    }
}

#[derive(Debug)]
pub struct Module {
    pub id: usize,
    pub name: String,
    pub code: Vec<Token>,

    pub variables: HashMap<String, usize>,
    pub procedures: HashMap<String, usize>,

    pub expressions: Vec<usize>,
}

impl Module {
    pub fn new(id: usize, name: String, code: Vec<Token>) -> Self {
        Self {
            id, name, code,

            variables: HashMap::new(),
            procedures: HashMap::new(),

            expressions: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, code: Token) {
        let name = code.sexpr().unwrap().get(1).unwrap().identifier().unwrap().clone();
        self.code.push(code);
        self.variables.insert(name, self.code.len() - 1);
    }

    pub fn add_procedure(&mut self, code: Token) {
        let name = code.sexpr().unwrap().get(1).unwrap().identifier().unwrap().clone();
        self.code.push(code);
        self.procedures.insert(name, self.code.len() - 1);
    }

    pub fn add_expression(&mut self, code: Token) {
        self.code.push(code);
        self.expressions.push(self.code.len() - 1);
    }
}
