mod validate;
pub use validate::*;

// We need to be able to execute top level macros at some point
// Also have some way to do partial validation to a module for parts that have not been expanded
// yet
