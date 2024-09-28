use xylo::repl::repl;

use std::{
    fs::File,
    path::Path,
    io::prelude::*,
};

// TODO: Make a proper ast from S-Expressions
// TODO: The lexer should produce S-Expressions
// TODO: Improve error messages, add more metadata to ast and s-expressions.

// TODO: Architecture change.
// 1. The sexpr generator will take the text and put them into sexprs of tokens.
// 2. it will keep Sweet and Type expressions as Tokens
// 3. It will also parse and convert Literals
// 4. First parse the imports.
// 5. Then the compiler will take the result and find infix declarations, this will also be done in the
//    sub modules.
// 6. Then parse all else + functions and their content.
// 7. The ast will contain sexp tokens because macros will need them.

// TODO: Add sweet expressions to make some things less verbose.
//       Keep in mind that this just gets translated to normal code by a preprosessor.

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
