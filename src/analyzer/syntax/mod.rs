mod pass1;
pub use pass1::*;

// We need to be able to execute top level macros at some point
// Also have some way to do partial validation to a module for parts that have not been expanded
// yet

// pass1: core syntax validation, collection of functions and macros
// pass2: function/macro syntax pass
