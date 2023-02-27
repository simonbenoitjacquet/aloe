use crate::term::Term;
use crate::atom::Atom;
use crate::predicate::Predicate;
use crate::substitution::Substitution;
use crate::knowledge::Knowledge;
use std::collections::VecDeque;

// #[derive(Clone)]
struct State {
    query: VecDeque<Atom>,
    substitution: Substitution,
    depth: u32,
}

fn rename_variables(depth: u32) -> Box<dyn Fn(&Term) -> Term> {
    Box::new(move |term: &Term| {
        match term {
            Term::Variable(name) => {
                let new_name: String = format!("{}_{}", name.clone(), depth);
                Term::Variable(new_name)
            },
            _ => return term.clone(), 
        }
    })
}

pub fn query(fact: &Atom, knowledge: &Knowledge) -> Option<Vec<Substitution>> {
    let state0 = State {
        query: VecDeque::from([fact.clone()]),
        substitution: Substitution::new(),
        depth: 0,
    };
    let mut states = VecDeque::from([state0]);
    let mut solutions: Vec<Substitution> = vec![];

    while !states.is_empty() {
        let State { mut query, substitution, depth } = states.pop_front().unwrap();

        let fact = match query.pop_front() {
            None => {
                // When there are no fact to query, we found ourselves a solution
                solutions.push(substitution);
                continue
            },
            Some(fact) => fact,
        };

        for clause in knowledge.get_clauses() {
            // Change variable names of clause
            let clause = clause.apply_on_elements(&rename_variables(depth+1));

            // Unify head of clause and fact
            let returned_subst = match Substitution::unify_atom(&clause.head, &fact) {
                Err(_) => continue,
                Ok(returned_subst) => returned_subst,
            };

            // Merge all substitutions
            let mut new_subst = substitution.clone();
            if let Err(_) = new_subst.merge(&returned_subst) {
                continue
            };

            // Apply substitution on query and on p.body
            let mut new_query: VecDeque<Atom> = query.iter().map(|x| new_subst.apply_on_atom(x)).collect();
            for atom in clause.body.iter().rev().map(|x| new_subst.apply_on_atom(x)){
                new_query.push_back(atom);
            }

            // Add new state
            states.push_front(State { query: new_query, substitution: new_subst, depth: depth+1 })
        }
    }

    if solutions.len() > 0 { 
        Some(solutions)
    } else {
        None
    }
}
