use crate::atom::Atom;

#[derive(Debug)]
pub struct Predicate {
    lhs: Atom,
    rhs: Vec<Atom>,
}

impl Predicate {
    pub fn new(lhs: Atom, rhs:Vec<Atom>) -> Self {
        Predicate { lhs, rhs }
    }
    pub fn matching(lhs: &Predicate, rhs: &Predicate) -> bool {
        if ! Atom::matching(&lhs.lhs, &rhs.lhs) || lhs.rhs.len() != rhs.rhs.len() {
            return false
        }
        for i in 0..lhs.rhs.len() {
            if ! Atom::matching(&lhs.rhs[i], &rhs.rhs[i]) {
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
        let atom1 = Atom::new (
            String::from("a"),
            Vec::new(),
        );
        let atom2 = Atom::new (
            String::from("a"),
            vec![
                Term::Function{
                    functor: String::from("fun"),
                    params: vec![
                        Term::Function{
                            functor: String::from("fun2"),
                            params: vec![Term::StrCst(String::from("age"))],
                        }
                    ],
                }
            ],
        );
        assert!(Atom::matching(&atom1, &atom1));
        assert!(Atom::matching(&atom2, &atom2));
        assert!(!Atom::matching(&atom1, &atom2));
    }
}
