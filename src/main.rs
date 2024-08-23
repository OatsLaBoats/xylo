use xylo::{
    lexer::Lexer,
    parser::Parser,
};

use std::{
    fs::File,
    path::Path,
    io::{
        self,
        prelude::*
    },
};

fn main() {
    let path = Path::new("test.xl");
    let path_text = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", path_text, e),
        Ok(file) => file,
    };

    let mut s = Vec::new();
    file.read_to_end(&mut s).unwrap();

    repl();
}

fn repl() {
    let mut lexer = Lexer::new();
    let mut parser = Parser::new();

    let mut input = String::new();

    loop {
        input.clear();

        print!("> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);

        let tokens = lexer.tokenize(&input.as_bytes());
        parser.parse(&tokens);

        println!("{:?}", tokens);
    }
}
