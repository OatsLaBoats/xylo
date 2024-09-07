use xylo::{
    lexer::lex,
    parser::parse,
    error::Error,
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

// TODO: There is probably a cleaner way to handle errors here but for now this will do.
fn repl() {
    let mut input = String::new();
    let mut errors: Vec<Error> = Vec::new();

    loop {
        if errors.len() > 0 {
            println!("{:?}", errors);
        }

        input.clear();
        errors.clear();

        print!("> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);

        let lex_result = lex(&input.as_bytes());
        if lex_result.is_err() {
            errors.extend(lex_result.unwrap_err());
            continue;
        }

        let tokens = lex_result.unwrap();
        let parse_result = parse(&tokens);
        if parse_result.is_err() {
            errors.extend(parse_result.unwrap_err());
            continue;
        }

        let sexps = parse_result.unwrap();
        println!("{:?}", sexps);
    }
}
