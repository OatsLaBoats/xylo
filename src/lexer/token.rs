#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(Debug)]
pub enum TokenKind {
    LParen,
    RParen,
    Number(NumberKind),
    String,
    Identifier,
}

#[derive(Debug)]
pub enum NumberKind {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
    Float,
}
