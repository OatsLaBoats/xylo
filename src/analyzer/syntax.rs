use crate::{
    program::*,
    token::*,
    utils::*,
};

// We need to be able to execute top level macros at some point
// Also have some way to do partial validation to a module for parts that have not been expanded
// yet

// pass1: Top level validation and collection of functions and macros
// pass2: Inner validation

// Checks top level constructs for syntax errors + collection of functions
pub fn syntax_pass1(program: &mut Program) -> Vec<Error> {
    let mut errors = Vec::new();

    let modules = program.get_modules_mut();
    for module in modules {
        pass1_check_module(module, &mut errors);
    }

    return errors;
}

fn pass1_check_module(module: &mut Module, errors: &mut Vec<Error>) {
    for token in &mut module.code {
        if let TokenKind::SExpr(sexp) = &mut token.kind {
            if sexp.get(0).map_or(false, |t| t.match_identifier("function")) {
                pass1_check_function(sexp, errors);
            }
        }
    }
}

fn pass1_check_function(sexp: &mut Vec<Token>, errors: &mut Vec<Error>) {
    let has_name = sexp.get(1).map_or(false, |t| t.is_identifier());
    let has_type = sexp.get(2).map_or(false, |t| t.is_type());

    let mut has_params = false;
    let mut has_valid_params = true;
    if let Some(token) = sexp.get(3) {
        if let TokenKind::SExpr(params) = &token.kind {
            has_params = true;

            for param in params {
                if !param.is_identifier() {
                    has_valid_params = false;
                }
            }
        }
    }

    let si = sexp.get(0).unwrap().si;

    if !has_name {
        errors.push(Error {
            message: "Functions require a name".to_string(),
            si,
        });
    }

    if !has_type {
        
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
}
