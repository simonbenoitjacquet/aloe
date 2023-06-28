use aloelib::{
    predicate::Predicate,
    term::Term,
    atom::Atom,
    query::query,
    knowledge::Knowledge,
    parser::*,
};

// mod predicate;
// mod atom;
// mod term;
// mod query;
// mod knowledge;

fn main() {
    let txt = "human(socrates)";
    println!("Original: {}", txt);
    let term = parse_term(txt);
    println!("Parsed: {:#?}", term);

    let txt = "human(socrates)";
    println!("Original: {}", txt);
    let term = parse_atom(txt);
    println!("Parsed: {:#?}", term);

    let txt = "human(socrates).";
    println!("Original: {}", txt);
    let term = parse_predicate(txt);
    println!("Parsed: {:#?}", term);
}
