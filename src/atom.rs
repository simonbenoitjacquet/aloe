use crate::term::Term;

#[derive(Debug, Clone)]
pub struct Atom {
    pub functor: String,
    pub params: Vec<Term>,
}

impl Atom {
    pub fn apply(&self, f: &dyn Fn(&Term) -> Term) -> Self {
        Atom {
            functor: self.functor.clone(),
            params: self.params.iter().map(|term| term.apply(f)).collect()
        }
    }

    pub fn apply_on_elements(&self, f: &dyn Fn(&Term) -> Term) -> Self {
        Atom {
            functor: self.functor.clone(),
            params: self.params.iter().map(|term| term.apply_on_elements(f)).collect()
        }
    }
    
    pub fn new(name: String, params: Vec<Term>) -> Self {
        Atom { functor: name, params }
    }

    pub fn matching(lhs: &Atom, rhs: &Atom) -> bool {
        if lhs.functor != rhs.functor || lhs.params.len() != rhs.params.len() {
            return false
        }
        for i in 0..lhs.params.len() {
            if ! Term::matching(&lhs.params[i], &rhs.params[i]) {
                return false
            }
        }
        true
    }

    pub fn get_params(&self) -> &Vec<Term> {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_1() {
        let atom1 = Atom::new (
            String::from("a"),
            Vec::new(),
        );
        let atom2 = Atom::new (
            String::from("a"),
            vec![
                Term::Function{
                    functor: String::from("fun"),
                    params: vec![Term::StrCst(String::from("age"))],
                }
            ],
        );
        assert!(Atom::matching(&atom1, &atom1));
        assert!(Atom::matching(&atom2, &atom2));
        assert!(!Atom::matching(&atom1, &atom2));
    }
}
