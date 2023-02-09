#[derive(Debug)]
enum Term {
    NumCst(i32),
    StrCst(String),
    Variable(String),
    Function {
        functor: String,
        params: Vec<Term>,
    },
}

#[derive(Debug)]
struct Atom {
    name: String,
    params: Vec<Term>,
}

#[derive(Debug)]
struct Predicate {
    lhs: Atom,
    rhs: Vec<Atom>,
}

impl Term {
    fn matching(lhs: &Term, rhs: &Term) -> bool {
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

impl Atom {
    fn matching(lhs: &Atom, rhs: &Atom) -> bool {
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

impl Predicate {
    fn matching(lhs: &Predicate, rhs: &Predicate) -> bool {
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


fn main() {
    let fun = Term::Function{
        functor: String::from("fun"),
        params: Vec::new(),
    };
    let fun2 = Term::Function{
        functor: String::from("fun"),
        params: Vec::new(),
    };

    let pred = Predicate {
        lhs: Atom {
            name: String::from("bob"),
            params: vec![ Term::NumCst(3), 
            Term::StrCst(String::from("age")), 
            Term::Variable(String::from("age")), 
            Term::Function{
                functor: String::from("fun"),
                params: Vec::new(),
            },
            ]
        },
        rhs: Vec::new(),
    };

    println!("{}", Term::matching(&fun, &fun2));
    println!("{}", Term::matching(&fun2, &fun2));
    println!("{}", Predicate::matching(&pred, &pred));
}
