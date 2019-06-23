use crate::unnamed::Term as UnnamedTerm;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Var(String),
    Apply(Box<Term>, Box<Term>),
    Lambda(String, Box<Term>),
}

impl Term {
    pub fn var<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Term::Var(s.into())
    }

    pub fn apply(t1: Term, t2: Term) -> Self {
        Term::Apply(Box::new(t1), Box::new(t2))
    }

    pub fn lambda<S>(s: S, t: Term) -> Self
    where
        S: Into<String>,
    {
        Term::Lambda(s.into(), Box::new(t))
    }

    pub fn from_unnamed(unnamed: &UnnamedTerm) -> Self {
        let free_vars = LexographicalNames::new(&[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        ]);
        let bound_vars = LexographicalNames::new(&['x', 'y', 'z', 'w', 'u', 'v', 'r', 's', 't']);

        Self::from_unnamed_inner(unnamed, 0, &free_vars, &bound_vars)
    }

    fn from_unnamed_inner(
        unnamed: &UnnamedTerm,
        depth: usize,
        free_vars: &LexographicalNames,
        bound_vars: &LexographicalNames,
    ) -> Self {
        match unnamed {
            &UnnamedTerm::Var(i) => {
                if i < depth {
                    Self::var(bound_vars.get(depth - i))
                } else {
                    Self::var(free_vars.get(i - depth + 1))
                }
            },
            UnnamedTerm::Apply(t1, t2) => Self::apply(
                Self::from_unnamed_inner(t1, depth, free_vars, bound_vars),
                Self::from_unnamed_inner(t2, depth, free_vars, bound_vars),
            ),
            UnnamedTerm::Lambda(t) => Self::lambda(
                bound_vars.get(depth + 1),
                Self::from_unnamed_inner(t, depth + 1, free_vars, bound_vars),
            ),
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
            Lambda(x, t @ box Var(_)) => write!(f, "λ {} {}", x, t),
            Lambda(x, t @ box Lambda(_, _)) => write!(f, "λ {} {}", x, t),
            Lambda(x, t) => write!(f, "λ {} ({})", x, t),
        }
    }
}

struct LexographicalNames<'a> {
    base: &'a [char],
}

impl<'a> LexographicalNames<'a> {
    pub fn new(base: &'a [char]) -> Self {
        Self { base }
    }

    pub fn get(&self, index: usize) -> String {
        let mut index = index;
        let mut result = Vec::new();

        while index > 0 {
            index -= 1;

            let c = self.base[index % self.base.len()];
            result.push(c);

            index /= self.base.len();
        }

        result.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests;
