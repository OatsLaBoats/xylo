use crate::{
    utils::Error,
    lexer::lex,
};

use std::io::{
    self,
    prelude::*,
};

// TODO: There is probably a cleaner way to handle errors here but for now this will do.
pub fn repl() {
    let mut input = String::new();

    loop {
        input.clear();

        print!("> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);

        if input == ":exit\r\n".to_string() {
            break;
        }

        let lex_result = lex(input.as_bytes());
        if lex_result.is_err() {
            let errors = lex_result.unwrap_err();
            for e in &errors {
                println!("{}", e.to_string("repl"))
            }

            continue;
        }

        let tokens = lex_result.unwrap();
        println!("{:?}", tokens);
     }
}
