use xylo::repl::repl;

use std::{
    fs::File,
    path::Path,
    io::prelude::*,
};

// Move source info and Error to some different module it doesn't fit utils

// NOTE: Optimize everything later

// TODO: Make a proper ast from S-Expressions
// TODO: Improve error messages, add more metadata to ast and s-expressions.

// Architecture
// Lexer produces S-Expressions made up of tokens.
// Parser 1st pass Collect everything that you can.
// Parser 2nd pass translate sweet expressions.
// Parser 3rd pass execute macro expressions.
// Analyser pass
// Typechecker/inference pass

// It might not even need an ast
// We can probably perform a syntax pass on raw s-expressions and then work on them directly

// TODO: Add sweet expressions to make some things less verbose.
//       Keep in mind that this just gets translated to normal code by a preprosessor.

// TODO: Have a way to run S-expressions on their own but providing a module

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
