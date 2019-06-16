use super::Term;

/// Perform the substitution `term[var -> subs]`
pub fn substitute(term: &Term, var: usize, subs: &Term) -> Term {
    use Term::*;

    fn lower(term: &Term) -> Term {
        match term {
            Var(x) => Var(x.wrapping_sub(1)),
            Apply(t1, t2) => Term::apply(lower(&t1), lower(&t2)),
            Lambda(t) => Term::lambda(lower(&t)),
        }
    }

    fn raise(term: &Term) -> Term {
        match term {
            Var(x) => Var(x.wrapping_add(1)),
            Apply(t1, t2) => Term::apply(raise(&t1), raise(&t2)),
            Lambda(t) => Term::lambda(raise(&t)),
        }
    }

    match term {
        Var(x) if *x == var => subs.clone(),
        Var(x) => Var(*x),
        Apply(t1, t2) => Term::apply(substitute(&t1, var, subs), substitute(&t2, var, subs)),
        Lambda(t) => Term::lambda(raise(&substitute(&lower(&t), var, subs))),
    }
}

#[cfg(test)]
mod tests;
