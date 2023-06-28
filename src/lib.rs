pub use crate::{
    predicate::Predicate,
    term::Term,
    atom::Atom,
    substitution::Substitution,
    utils::*,
    query::*,
    knowledge::*,
    parser::*,
};

pub mod predicate;
pub mod atom;
pub mod term;
pub mod substitution;
pub mod utils;
pub mod query;
pub mod knowledge;
pub mod parser;

