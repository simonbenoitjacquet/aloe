use aloelib::{
    predicate::Predicate,
    term::Term,
    atom::Atom,
    query::query,
    knowledge::Knowledge,
    parser::*,
};

use dialoguer::Input;


fn main() -> Result<(), std::io::Error> {
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

    let fact = parse_atom("mortal(socrates)").unwrap();
    let mut knowledge = Knowledge::new();
    knowledge.add(parse_predicate("mortal(X):-man(X).").unwrap());
    knowledge.add(parse_predicate("man(socrates).").unwrap());
    let result = query(&fact, &knowledge);
    println!("Query result: {:#?}", result);


    let mut knowledge = Knowledge::new();
    println!("\n\nCLI for {}", "aloe");
    for _ in 1..10 {
        let mut input : String = Input::new()
            .interact_text()?;
        if input.chars().last().unwrap()=='?' {
            input.pop();
            let fact = parse_atom(input.as_str()).unwrap();
            let result = query(&fact, &knowledge);
            println!("Result: {:#?}", result);
        } else {
            knowledge.add(parse_predicate(input.as_str()).unwrap());
        }
    }

    Ok(())
}
