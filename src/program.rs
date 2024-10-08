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

    pub fn new_module(&mut self, name: String, code: Vec<Token>) {
        let id = self.modules.len();

        self.module_lookup.insert(name.clone(), id);
        self.modules.push(Module::new(id, name, code));
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
}

impl Module {
    pub fn new(id: usize, name: String, code: Vec<Token>) -> Self {
        Self {
            id, name, code,

            variables: HashMap::new(),
            procedures: HashMap::new(),
        }
    }
}
