use std::collections::HashMap;
use crate::term::Term;
use crate::atom::Atom;
use crate::predicate::Predicate;
use crate::utils::*;

#[derive(Clone)]
pub struct Substitution {
    data: HashMap<String, Term>,
}

impl Substitution {
    pub fn new() -> Self { 
        Self { data: HashMap::new() } 
    }

    pub fn from(var: String, val: Term) -> Self {
        let mut map = HashMap::new();
        map.insert(var, val);
        Self {
            data: map
        }
    }

    pub fn unify_atom<'a>(lhs: &Atom, rhs: &Atom) -> Result<Substitution, &'a str> {
        let Atom { functor: f1, params: p1 } = lhs;
        let Atom { functor: f2, params: p2 } = rhs;

        if f1 != f2 || p1.len() != p2.len() {
            return Err("Cannot unify atoms {:#?lhs} and {:#?rhs}")
        }
        let mut substitution = Substitution::new();
        for i in 0..p1.len() {
            let new_subst = Self::unify(&p1[i], &p2[i])?;
            substitution.merge(&new_subst)?;
        }
        Ok(substitution)
    }

    pub fn unify<'a>(lhs: &Term, rhs: &Term) -> Result<Substitution, &'a str> {
        match (lhs, rhs) {
            (Term::Variable(x), other_term) | (other_term, Term::Variable(x)) => {
                if variables_in_term(&other_term).contains(&x) {
                    Err("Cannot bind variable {x} to {other_term}")
                } else {
                    Ok(Substitution::from(x.clone(), other_term.clone()))
                }
            },
            (Term::NumCst(x), Term::NumCst(y)) => {
                match x.cmp(&y) {
                    std::cmp::Ordering::Equal => Ok(Substitution::new()),
                    _ => Err("Cannot unify constants {x} and {y}")
                }
            },
            (Term::StrCst(x), Term::StrCst(y)) => {
                match x.cmp(&y) {
                    std::cmp::Ordering::Equal => Ok(Substitution::new()),
                    _ => Err("Cannot unify constants {x} and {y}")
                }
            },
            (
                Term::Function{ functor: f1, params: p1 }, 
                Term::Function{ functor: f2, params: p2 }
            ) => {
                if f1 != f2 || p1.len() != p2.len() {
                    return Err("Cannot unify functions {:#?lhs} and {:#?rhs}")
                }
                let mut substitution = Substitution::new();
                for i in 0..p1.len() {
                    let new_subst = Self::unify(&p1[i], &p2[i])?;
                    substitution.merge(&new_subst)?;
                }
                Ok(substitution)
            },
            _ => Err("Terms {rhs} and {lhs} cannot be unified"),
        }
    }
    
    pub fn merge<'a>(&mut self, other: &Self) -> Result<(), &'a str> {
        for (o_key, o_value) in &other.data {
            for var in variables_in_term(o_value) {
                if let Some(var_term) = self.data.get(&var) {
                    if variables_in_term(&var_term).contains(&var) {
                        return Err("Cannot merge {self} into {other}")
                    }
                }
            }
            match self.data.get(o_key) {
                None => {
                    self.data.insert(o_key.clone(), o_value.clone());
                },
                Some(other_val) => {
                    let third_subst = Substitution::unify(&o_value, &other_val)?;
                    self.merge(&third_subst)?;
                }
            }
        }
        Ok(())
    }

    pub fn apply_on_term(&self, term:&Term) -> Term {
        match term {
            Term::Variable(name) => {
                match self.data.get(name) {
                    None => term.clone(),
                    Some(value) => value.clone(),
                }
            },
            Term::Function{functor, params} => {
                Term::Function{
                    functor: functor.clone(),
                    params: params.iter().map(|t| self.apply_on_term(t)).collect(),
                }
            },
            _ => term.clone(),
        }
    }

    pub fn apply_on_atom(&self, atom:&Atom) -> Atom {
        Atom {
            functor: atom.functor.clone(),
            params: atom.params.iter().map(|t| self.apply_on_term(t)).collect(),
        }
    }

    pub fn apply_on_predicate(&self, predicate:&Predicate) -> Predicate {
        Predicate {
            head: self.apply_on_atom(&predicate.head),
            body: predicate.body.iter().map(|t| self.apply_on_atom(t)).collect(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_1() {
        let str_cst = Term::StrCst(String::from("3"));
        let num_cst = Term::NumCst(3);
        let var = Term::Variable(String::from("X"));
        let var2 = Term::Variable(String::from("Y"));
        let fun = Term::Function{
            functor: String::from("fun"),
            params: vec![Term::StrCst(String::from("xx"))],
        };
        let fun2 = Term::Function{
            functor: String::from("fun"),
            params: vec![Term::StrCst(String::from("age"))],
        };
        let fun3 = Term::Function{
            functor: String::from("fun"),
            params: vec![Term::Variable(String::from("X"))],
        };

        assert!(Substitution::unify(&str_cst, &str_cst).is_ok());
        assert!(Substitution::unify(&num_cst, &num_cst).is_ok());
        assert!(Substitution::unify(&var, &var2).is_ok());
        assert!(Substitution::unify(&fun, &var2).is_ok());
        assert!(Substitution::unify(&fun2, &fun3).is_ok());
        assert!(Substitution::unify(&var, &fun3).is_err());
    }
}
