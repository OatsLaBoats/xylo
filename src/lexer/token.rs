#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    LParen,
    RParen,
    Number(NumberKind),
    Identifier,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberKind {
    Decimal,
    Base2,
    Base8,
    Base10,
    Base16,
}
