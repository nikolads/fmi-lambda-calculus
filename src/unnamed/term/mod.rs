use crate::named::Term as NamedTerm;
use std::fmt::{self, Display};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize)]
pub enum Term {
    Var(usize),
    Apply(Box<Term>, Box<Term>),
    Lambda(Box<Term>),
}

impl Term {
    pub fn var(n: usize) -> Self {
        Term::Var(n)
    }

    pub fn apply(t1: Term, t2: Term) -> Self {
        Term::Apply(Box::new(t1), Box::new(t2))
    }

    pub fn lambda(t: Term) -> Self {
        Term::Lambda(Box::new(t))
    }

    /// Create an unnamed term from a named term
    pub fn from_named(named: &NamedTerm) -> (Self, Vec<String>) {
        let mut names = Vec::new();
        let mut args = Vec::new();

        let term = Self::from_named_inner(named, &mut args, &mut names);
        (term, names)
    }

    fn from_named_inner(
        named: &NamedTerm,
        args: &mut Vec<String>,
        names: &mut Vec<String>,
    ) -> Self {
        match named {
            NamedTerm::Var(x) => match args.iter().rev().enumerate().find(|(_, arg)| **arg == *x) {
                Some((index, _)) => Self::var(index),
                None => match names.iter().position(|name| name == x) {
                    Some(index) => Self::var(index),
                    None => {
                        names.push(x.clone());
                        Self::var(args.len() + names.len() - 1)
                    },
                },
            },
            NamedTerm::Apply(t1, t2) => Self::apply(
                Self::from_named_inner(t1, args, names),
                Self::from_named_inner(t2, args, names),
            ),
            NamedTerm::Lambda(x, t) => {
                args.push(x.clone());
                let result = Self::lambda(Self::from_named_inner(t, args, names));
                args.pop();
                result
            },
        }
    }

    /// Perform the substitution `term[var -> subs]`
    pub fn substitute(&self, var: usize, subs: &Term) -> Term {
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

        match self {
            Var(x) if *x == var => subs.clone(),
            Var(x) => Var(*x),
            Apply(t1, t2) => Term::apply(t1.substitute(var, subs), t2.substitute(var, subs)),
            Lambda(t) => Term::lambda(raise(&lower(&t).substitute(var, subs))),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;

        match self {
            Var(x) => write!(f, "{}", x),
            Apply(t1, t2) => {
                match **t1 {
                    Var(_) | Apply(_, _) => write!(f, "{}", t1)?,
                    _ => write!(f, "({})", t1)?,
                }

                write!(f, " ")?;

                match **t2 {
                    Var(_) => write!(f, "{}", t2),
                    _ => write!(f, "({})", t2),
                }
            },
            Lambda(t) => match **t {
                Var(_) | Lambda(_) => write!(f, "λ {}", t),
                _ => write!(f, "λ ({})", t),
            },
        }
    }
}

#[cfg(test)]
mod tests;
