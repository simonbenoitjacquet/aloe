mod predicate;
mod atom;
mod term;
mod substitution;
mod utils;
mod query;
mod knowledge;
use crate::predicate::Predicate;
use crate::term::Term;
use crate::atom::Atom;
use crate::substitution::Substitution;
use crate::utils::*;
use crate::query::*;
use crate::knowledge::*;

fn main() {
    let fun = Term::Function{
        functor: String::from("fun"),
        params: Vec::new(),
    };
    let fun2 = Term::Function{
        functor: String::from("fun"),
        params: Vec::new(),
    };

    let pred = Predicate::new (
        Atom::new (
            String::from("bob"),
            vec![ 
            Term::NumCst(3), 
            Term::StrCst(String::from("age")), 
            Term::Variable(String::from("age")), 
            Term::Function{
                functor: String::from("fun"),
                params: Vec::new(),
            },
            ]
        ),
        Vec::new(),
    );

    println!("{}", Term::matching(&fun, &fun2));
    println!("{}", Term::matching(&fun2, &fun2));
    println!("{}", Predicate::matching(&pred, &pred));
}
