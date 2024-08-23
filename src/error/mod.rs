#[derive(Clone)]
pub struct Error {
    pub kind: ErrorKind,

    pub line: i32,
    pub column: i32,
    pub file: String,
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum ErrorKind {
    SyntaxError,
    SemanticError,
    TypeError,
}
