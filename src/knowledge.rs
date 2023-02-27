use crate::term::Term;
use crate::atom::Atom;
use crate::predicate::Predicate;
use crate::substitution::Substitution;

pub struct Knowledge {
    clauses: Vec<Predicate>
}

impl Knowledge {
    pub fn new() -> Self { Knowledge { clauses: vec![] } }

    pub fn add(&mut self, predicate: &Predicate){
        self.clauses.push(predicate.clone());
    }

    pub fn get_clauses(&self) -> &Vec<Predicate>{
        &self.clauses
    }

    pub fn matching(&self, atom: &Atom) -> Option<Vec<Predicate>> {
        let matched: Vec<Predicate> = self.clauses
            .iter()
            .filter(|p| p.matching_head(&atom))
            .map(|c| c.clone())
            .collect();
        if matched.len() == 0 {
            None
        } else {
            Some(matched)
        }
    }
}
