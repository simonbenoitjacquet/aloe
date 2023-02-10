use crate::term::Term;

#[derive(Debug)]
pub struct Atom {
    name: String,
    params: Vec<Term>,
}

impl Atom {
    pub fn new(name: String, params: Vec<Term>) -> Self {
        Atom { name, params }
    }

    pub fn matching(lhs: &Atom, rhs: &Atom) -> bool {
        if lhs.name != rhs.name || lhs.params.len() != rhs.params.len() {
            return false
        }
        for i in 0..lhs.params.len() {
            if ! Term::matching(&lhs.params[i], &rhs.params[i]) {
                return false
            }
        }
        true
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
