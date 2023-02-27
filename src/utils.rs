use crate::term::Term;
use crate::atom::Atom;

pub fn variables_in_term(x: &Term) -> Vec<String>{
    match x {
        Term::Variable(s) => return vec![s.clone()],
        Term::Function {
            functor: _,
            params,
        } => {
            let mut all_vars = vec![];
            for term in params {
                all_vars.extend(variables_in_term(&term));
            }
            all_vars
        },
        _ => return vec![],
    }
}

pub fn variables_in_atom(x: &Atom) -> Vec<String>{
    let mut all_vars = vec![];
    for term in x.get_params() {
        all_vars.extend(variables_in_term(&term));
    }
    all_vars
}
