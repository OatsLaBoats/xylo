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
pub fn pass1(program: &mut Program) -> Vec<Error> {
    let mut errors = Vec::new();

    let modules = program.get_modules_mut();
    for module in modules {
        pass1_check_module(module, &mut errors);
    }

    return errors;
}

fn pass1_check_module(module: &mut Module, errors: &mut Vec<Error>) {
    for (index, token) in module.code.iter_mut().enumerate() {
        if let TokenKind::SExpr(sexp) = &mut token.kind {
            if sexp.get(0).map_or(false, |t| t.match_identifier("function")) {
                if pass1_check_function(sexp, errors) {
                    let name = sexp.get(1).unwrap().get_identifier();
                    module.variables.insert(name.clone(), index);
                }
            }
        }
    }
}

fn pass1_check_function(sexp: &mut Vec<Token>, errors: &mut Vec<Error>) -> bool {
    let si = sexp.get(0).unwrap().si;

    let has_name = sexp.get(1).map_or(false, |t| t.is_identifier());
    let has_type = sexp.get(2).map_or(false, |t| t.is_type());

    let mut has_params = false;
    let mut has_valid_params = true;

    let params_index = if has_type { 3 } else { 2 };

    if let Some(token) = sexp.get(params_index) {
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

    // Insert an unknown type if its missing to infer it during type checking
    if !has_type && has_params && has_valid_params {
        sexp.insert(2, Token {
            kind: TokenKind::TypeExpr(Type::Unknown),
            si: SourceInfo::default(),
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

    return has_name && has_params && has_valid_params;
}
