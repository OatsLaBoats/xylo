use crate::token::*;

pub fn is_variable(token: &Token) -> bool {
    token.match_first_identifier("let")
}

pub fn is_procedure(token: &Token) -> bool {
    token.match_first_identifier("procedure")
}

pub fn is_lambda(token: &Token) -> bool {
    token.match_first_identifier("fun")
}
