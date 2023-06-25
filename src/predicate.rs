use crate::atom::Atom;
use crate::term::Term;

#[derive(Debug, Clone)]
pub struct Predicate {
    pub head: Atom,
    pub body: Vec<Atom>,
}

impl Predicate {
    pub fn apply(&self, f: &dyn Fn(&Term) -> Term) -> Self {
        Predicate {
            head: self.head.apply(f),
            body: self.body.iter().map(|atom| atom.apply(f)).collect()
        }
    }
    
    pub fn apply_on_elements(&self, f: &dyn Fn(&Term) -> Term) -> Self {
        Predicate {
            head: self.head.apply_on_elements(f),
            body: self.body.iter().map(|atom| atom.apply_on_elements(f)).collect()
        }
    }
    
    pub fn matching_head(&self, atom: &Atom) -> bool {
        Atom::matching(&self.head, &atom)
    }

    pub fn matching(lhs: &Predicate, rhs: &Predicate) -> bool {
        if ! Atom::matching(&lhs.head, &rhs.head) || lhs.body.len() != rhs.body.len() {
            return false
        }
        for i in 0..lhs.body.len() {
            if ! Atom::matching(&lhs.body[i], &rhs.body[i]) {
                return false
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::Term;

    #[test]
    fn test_matching_1() {
        let term1 = Term::Function {
            functor: String::from("fun2"),
            params: vec![Term::StrCst(String::from("age"))],
        };
        let term2 = Term::Function {
            functor: String::from("fun"),
            params: vec![ term1 ],
        };
        let atom1 = Atom {
            functor: String::from("a"),
            params: vec![],
        };
        let atom2 = Atom {
            functor: String::from("a"),
            params: vec![ term2 ],
        };
        assert!(Atom::matching(&atom1, &atom1));
        assert!(Atom::matching(&atom2, &atom2));
        assert!(!Atom::matching(&atom1, &atom2));
    }
}
