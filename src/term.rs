#[derive(Debug)]
pub enum Term {
    NumCst(i32),
    StrCst(String),
    Variable(String),
    Function {
        functor: String,
        params: Vec<Term>,
    },
}

impl Term {
    pub fn matching(lhs: &Term, rhs: &Term) -> bool {
        match (lhs, rhs) {
            (Term::Variable(_), _) => true,
            (_, Term::Variable(_)) => true,
            (Term::NumCst(x), Term::NumCst(y)) => x == y,
            (Term::StrCst(x), Term::StrCst(y)) => x == y,
            (
                Term::Function{ functor: f1, params: p1 }, 
                Term::Function{ functor: f2, params: p2 }
            ) => {
                if f1 != f2 || p1.len() != p2.len() {
                    return false 
                }
                for i in 0..p1.len() {
                    if ! Self::matching(&p1[i], &p2[i]) {
                        return false
                    }
                }
                true
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_1() {
        let str_cst = Term::StrCst(String::from("3"));
        let str_cst2 = Term::StrCst(String::from("val"));
        let num_cst = Term::NumCst(3);
        let num_cst2 = Term::NumCst(5);
        let var = Term::Variable(String::from("X"));
        let var2 = Term::Variable(String::from("Y"));
        let fun = Term::Function{
            functor: String::from("fun"),
            params: Vec::new(),
        };
        let fun2 = Term::Function{
            functor: String::from("fun"),
            params: vec![Term::StrCst(String::from("age"))],
        };

        assert!(Term::matching(&str_cst, &str_cst), "Identity match for StrCst");
        assert!(Term::matching(&num_cst, &num_cst), "Identity match for NumCst");
        assert!(Term::matching(&var, &var), "Identity match for Variable");
        assert!(Term::matching(&fun, &fun), "Identity match for Function");

        assert!(Term::matching(&var, &var2));
        assert!(Term::matching(&var, &str_cst));
        assert!(Term::matching(&fun, &var));

        assert!(!Term::matching(&str_cst, &str_cst2));
        assert!(!Term::matching(&num_cst, &num_cst2));
        assert!(!Term::matching(&fun, &fun2));
        assert!(!Term::matching(&str_cst, &num_cst));
        assert!(!Term::matching(&fun, &str_cst));
    }
}
