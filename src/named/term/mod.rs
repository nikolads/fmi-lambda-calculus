use crate::unnamed::Term as UnnamedTerm;
use serde_derive::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fmt::{self, Display};

/// Ламбда терм
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Term {
    Var(String),
    Apply(Box<Term>, Box<Term>),
    Lambda(String, Box<Term>),
}

impl Term {
    const FV_LETTERS: [char; 17] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    ];
    const ARG_LETTERS: [char; 9] = ['x', 'y', 'z', 'w', 'u', 'v', 'r', 's', 't'];

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

    /// Превръща безименен ламбда терм в именуван.
    ///
    /// Използва автоматично генериран контекст от имена.
    pub fn from_unnamed(unnamed: &UnnamedTerm) -> Self {
        let free_vars = LexicographicalNames::new(&Self::FV_LETTERS);
        let bound_vars = LexicographicalNames::new(&Self::ARG_LETTERS);

        Self::from_unnamed_inner(unnamed, 0, &free_vars, &bound_vars)
    }

    fn from_unnamed_inner(
        unnamed: &UnnamedTerm,
        depth: usize,
        free_vars: &LexicographicalNames,
        bound_vars: &LexicographicalNames,
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

    /// Изпълнява субституцията `term[var -> subs]`
    pub fn substitute(&self, var: &str, subs: &Term) -> Term {
        let fv_subs = subs.free_vars();

        match self {
            Term::Var(x) if x == var => subs.clone(),
            Term::Var(x) => Term::var(x.clone()),
            Term::Apply(t1, t2) => Term::apply(t1.substitute(var, subs), t2.substitute(var, subs)),
            Term::Lambda(x, t) if x == var => Term::Lambda(x.clone(), t.clone()),
            Term::Lambda(x, t) if fv_subs.get(x).is_some() => {
                let fv_term = t.free_vars();

                let name_generator = LexicographicalNames::new(&Self::ARG_LETTERS);
                let name = (1..)
                    .map(|i| name_generator.get(i))
                    .skip_while(|name| fv_subs.get(name).is_some() || fv_term.get(name).is_some())
                    .next()
                    .unwrap();

                let term = t.substitute(x, &Term::var(name.clone())).substitute(var, subs);
                Term::lambda(name, term)
            },
            Term::Lambda(x, t) => Term::lambda(x.clone(), t.substitute(var, subs)),
        }
    }

    /// Връща свободните променливи на терма
    fn free_vars(&self) -> HashSet<String> {
        let mut fv = HashSet::new();
        self.fill_free_vars(&mut vec![], &mut fv);
        fv
    }

    fn fill_free_vars<'a>(&'a self, args: &mut Vec<&'a str>, fv: &mut HashSet<String>) {
        match self {
            Term::Var(x) => {
                if args.iter().find(|arg| **arg == *x).is_none() {
                    fv.insert(x.clone());
                }
            },
            Term::Apply(t1, t2) => {
                t1.fill_free_vars(args, fv);
                t2.fill_free_vars(args, fv);
            },
            Term::Lambda(x, t) => {
                args.push(x);
                t.fill_free_vars(args, fv);
                args.pop();
            },
        }
    }
}

/// Формат за принтиране.
///
/// Използва се от `println!("{}", ...)`
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
            Lambda(x, t) => write!(f, "λ {}. {}", x, t),
        }
    }
}

/// Генерира имена на променливи.
///
/// Имената се генерират от списък със символи - `base` и са подредени първо по
/// дължина, после лексикографски.
///
/// # Пример
///
/// ```ignore
/// let gen = LexicographicalNames::new(&['a', 'b', 'c']);
///
/// assert_eq!(
///     (0..15).map(|i| gen.get(i)).collect::<Vec<_>>(),
///     vec!["", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab"]
/// );
/// ```
struct LexicographicalNames<'a> {
    base: &'a [char],
}

impl<'a> LexicographicalNames<'a> {
    pub fn new(base: &'a [char]) -> Self {
        LexicographicalNames { base }
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
