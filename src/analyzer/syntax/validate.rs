use crate::{
    program::*,
    token::*,
    utils::*,
};

// Checks top level constructs for syntax errors + collection of functions
pub fn validate(program: &mut Program) -> Vec<Error> {
    let mut errors = Vec::new();

    for module in program.get_modules_mut() {
        validate_module(module, &mut errors);
    }

    return errors;
}

// For repl use
pub fn validate_code(code: &mut Vec<Token>) -> Vec<Error> {
    let mut errors = Vec::new();

    for token in code {
        validate_token(token, &mut errors);
    }

    return errors;
}

fn validate_module(module: &mut Module, errors: &mut Vec<Error>) {
    for (index, token) in module.code.iter_mut().enumerate() {
        if token.is_sexpr() {
            let n_errors = errors.len();
            validate_sexpr(token, errors);
            
            if errors.len() == n_errors {
                if token.match_first_identifier("let") {
                    let name = token.sexpr().unwrap().get(1).unwrap().identifier().unwrap();
                    module.variables.insert(name.clone(), index);
                } else if token.match_first_identifier("procedure") {
                    let name = token.sexpr().unwrap().get(1).unwrap().identifier().unwrap();
                    module.procedures.insert(name.clone(), index);
                }
            }
        } else {
            module.expressions.push(index);
        }
    }
}

fn validate_token(token: &mut Token, errors: &mut Vec<Error>) {
    if token.is_sexpr() {
        validate_sexpr(token, errors);
    }
}

fn validate_sexpr(sexpr: &mut Token, errors: &mut Vec<Error>) {
    if sexpr.match_first_identifier("function") {
        validate_function(sexpr, errors);
    } else if sexpr.match_first_identifier("procedure") {
        validate_procedure(sexpr, errors);
    } else if sexpr.match_first_identifier("let") {
        validate_let(sexpr, errors);
    } else if sexpr.match_first_identifier("fun") {
        validate_fun(sexpr, errors);
    } else {
        validate_call(sexpr, errors);
    }
}

fn validate_call(sexpr: &mut Token, errors: &mut Vec<Error>) {
    let sexpr = sexpr.sexpr_mut().unwrap();
    if sexpr.len() > 1 {
        for token in &mut sexpr[1..] {
            validate_token(token, errors);
        }
    }
}

fn validate_fun(sexpr: &mut Token, errors: &mut Vec<Error>) {
    let sexpr = sexpr.sexpr_mut().unwrap();
    let si = sexpr[0].si;

    let mut has_params = false;
    let mut has_valid_params = true;

    if let Some(token) = sexpr.get(1) {
        if let TokenKind::SExpr(params) = &token.kind {
            has_params = true;
            for p in params {
                if !p.is_identifier() {
                    has_valid_params = false;
                    break;
                }
            }
        }
    }

    if !has_params {
        errors.push(Error {
            message: "Lambda functions require a parameter list".to_string(),
            si,
        });
    } else if !has_valid_params {
        errors.push(Error {
            message: "Paramaters need to be valid identifiers".to_string(),
            si,
        });
    }

    if sexpr.len() >= 3 {
        for token in &mut sexpr[3..] {
            validate_token(token, errors);
        }
    } else {
        errors.push(Error {
            message: "Lambda functions require at least one expression in them".to_string(),
            si,
        });
    }
}

fn validate_let(sexpr: &mut Token, errors: &mut Vec<Error>) {
    let sexpr = sexpr.sexpr_mut().unwrap();
    let si = sexpr[0].si;

    let has_name = sexpr.get(1).map_or(false, |t| t.is_identifier());
    let has_type = sexpr.get(2).map_or(false, |t| t.is_type());

    if !has_name {
        errors.push(Error {
            message: "Variables require a name".to_string(),
            si,
        });
    }

    if !has_type {
        sexpr.insert(2, Token {
            kind: TokenKind::TypeExpr(Type::Unknown),
            si: SourceInfo::default(),
        });
    }

    if let Some(value) = sexpr.get_mut(3) {
        validate_token(value, errors);
    } else {
        errors.push(Error {
            message: "Variables require an initial value".to_string(),
            si,
        });
    }
}

fn validate_procedure(sexpr: &mut Token, errors: &mut Vec<Error>) {
    let sexpr = sexpr.sexpr_mut().unwrap();
    let si = sexpr[0].si;

    let has_name = sexpr.get(1).map_or(false, |t| t.is_identifier());
    let has_type = sexpr.get(2).map_or(false, |t| t.is_type());

    if !has_name {
        errors.push(Error {
            message: "Procedures require a name".to_string(),
            si,
        });
    }

    if !has_type {
        sexpr.insert(2, Token {
            kind: TokenKind::TypeExpr(Type::Unknown),
            si: SourceInfo::default(),
        });
    }

    if sexpr.len() >= 3 {
        for token in &mut sexpr[3..] {
            validate_token(token, errors);
        }
    }
}

fn validate_function(sexpr: &mut Token, errors: &mut Vec<Error>) {
    let sexpr = sexpr.sexpr_mut().unwrap();
    let si = sexpr[0].si;

    let has_name = sexpr.get(1).map_or(false, |t| t.is_identifier());
    let has_type = sexpr.get(2).map_or(false, |t| t.is_type());

    let mut has_params = false;
    let mut has_valid_params = true;

    let params_index = if has_type { 3 } else { 2 };

    if let Some(token) = sexpr.get(params_index) {
        if let TokenKind::SExpr(params) = &token.kind {
            has_params = true;
            for param in params {
                if !param.is_identifier() {
                    has_valid_params = false;
                    break;
                }
            }
        }
    }

    if !has_name {
        errors.push(Error {
            message: "Functions require a name".to_string(),
            si,
        });
    }

    if !has_params {
        errors.push(Error {
            message: "Functions are required to have at least one parameter".to_string(),
            si,
        });
    } else if !has_valid_params {
        errors.push(Error {
            message: "Function parameters have to be identifiers".to_string(),
            si,
        });
    }

    if has_params && has_valid_params {
        if !has_type {
            sexpr.insert(2, Token {
                kind: TokenKind::TypeExpr(Type::Unknown),
                si: SourceInfo::default(),
            });
        }

        sexpr[0] = Token {
            kind: TokenKind::Identifier("let".to_string()),
            si,
        };

        let mut body = vec![
            Token { kind: TokenKind::Identifier("fun".to_string()), si: SourceInfo::default() },
            sexpr.remove(3),
        ];

        for i in (3..sexpr.len()).rev() {
            body.push(sexpr.remove(i));
        }

        body[2..].reverse();
        sexpr.push(Token { kind: TokenKind::SExpr(body), si: SourceInfo::default() });
    }
    
    validate_sexpr(&mut sexpr[3], errors);
}
