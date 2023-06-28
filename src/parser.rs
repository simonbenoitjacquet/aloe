use crate::{
    predicate::Predicate,
    term::Term,
    atom::Atom,
    knowledge::Knowledge,
};

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "aloe_grammar.pest"]
struct AloeParser;

// Visit functions

fn visit_term<'a>(rule : Pair<Rule>) -> Result<Term, &'a str> {
    match rule.as_rule() {
        Rule::term => {
            let subrule = rule.into_inner().next().unwrap();
            visit_term(subrule)
        }
        Rule::function => {
            let mut functor = String::new();
            let mut params : Vec<Term> = vec![];
            for subrule in rule.into_inner() {
                match subrule.as_rule() {
                    Rule::name => functor = subrule.as_str().to_string(),
                    Rule::term => {
                        let term = visit_term(subrule)?;
                        params.push(term);
                    }
                    _ => return Err("Error"),
                }
            }
            Ok(Term::Function{ functor, params })
        },
        Rule::strcst => {
            let value = rule.as_str().to_string();
            Ok(Term::StrCst(value))
        },
        Rule::numcst => {
            let value = rule.as_str()
                .to_string()
                .parse::<i32>()
                .unwrap();
            Ok(Term::NumCst(value))
        },
        Rule::variable => {
            let value = rule.as_str().to_string();
            Ok(Term::Variable(value))
        },
        _ => Err("Error"),
    }
}
fn visit_atom<'a>(rule : Pair<Rule>) -> Result<Atom, &'a str> {
    match rule.as_rule() {
        Rule::atom => {
            let mut functor = String::new();
            let mut params : Vec<Term> = vec![];
            for subrule in rule.into_inner() {
                match subrule.as_rule() {
                    Rule::name => functor = subrule.as_str().to_string(),
                    Rule::term => {
                        let term = visit_term(subrule)?;
                        params.push(term);
                    }
                    _ => return Err("Error"),
                }
            }
            Ok(Atom{ functor, params })
        },
        _ => Err("Error"),
    }
}
fn visit_predicate<'a>(rule : Pair<Rule>) -> Result<Predicate, &'a str> {
    match rule.as_rule() {
        Rule::predicate => {
            let mut head : Option<Atom> = None;
            let mut body : Vec<Atom> = vec![];
            for subrule in rule.into_inner() {
                match subrule.as_rule() {
                    Rule::atom => {
                        let atom = visit_atom(subrule)?;
                        if head.is_none() {
                            head = Some(atom);
                        } else {
                            body.push(atom);
                        }
                    },
                    _ => return Err("Error"),
                }
            }
            match head {
                None => Err("Error"),
                Some(h) => Ok(Predicate { head:h, body }),
            }
        },
        _ => Err("Error"),
    }
}

// Parse functions

pub fn parse_term<'a>(text : &str) -> Result<Term, &'a str> {
    let pair = AloeParser::parse(Rule::term, text)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    visit_term(pair)
}
pub fn parse_atom<'a>(text : &str) -> Result<Atom, &'a str> {
    let pair = AloeParser::parse(Rule::atom, text)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    visit_atom(pair)
}
pub fn parse_predicate<'a>(text : &str) -> Result<Predicate, &'a str> {
    let pair = AloeParser::parse(Rule::predicate, text)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    visit_predicate(pair)
}
