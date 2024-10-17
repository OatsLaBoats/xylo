use crate::{
    tokenizer::tokenize,
    program::*,
    analyzer::{
        syntax,
        utils::*,
    },
};

use std::io::{
    self,
    prelude::*,
};

// TODO: There is probably a cleaner way to handle errors here but for now this will do.

pub fn repl() {
    let mut input = String::new();
    let mut program = Program::new();

    let module_id = program.new_module("repl".to_string(), Vec::new());
    let module = program.get_module_by_id_mut(module_id).unwrap();

    loop {
        input.clear();

        print!("> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);

        if input == ":exit\r\n".to_string() {
            break;
        }

        let lex_result = tokenize(input.as_bytes());
        if lex_result.is_err() {
            let errors = lex_result.unwrap_err();
            for e in &errors {
                println!("{}", e.to_string("repl"))
            }

            continue;
        }

        let mut tokens = lex_result.unwrap();

        let errors = syntax::pass1_code(&mut tokens);
        if errors.len() == 0 {
            for token in tokens {
                if is_variable(&token) {
                    module.add_variable(token);
                } else if is_procedure(&token) {
                    module.add_procedure(token);
                }
            }
        }

        println!("{:?}", errors);

        for var in &module.code {
            print!("{}\n\n", var.to_string());
        }
     }
}
