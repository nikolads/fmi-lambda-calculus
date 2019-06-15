use std::fmt::{self, Display};

#[derive(Debug, Clone, Eq, PartialEq)]
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
            Lambda(t @ box Var(_)) => write!(f, "λ {}", t),
            Lambda(t @ box Lambda(_)) => write!(f, "λ {}", t),
            Lambda(t) => write!(f, "λ ({})", t),
        }
    }
}
