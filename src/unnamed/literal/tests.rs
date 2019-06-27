use crate::unnamed::Term;
use crate::unnamed_term;

#[test]
fn macro_var() {
    assert_eq!(unnamed_term!(0), Term::var(0));
    assert_eq!(unnamed_term!((1)), Term::var(1));
}

#[test]
fn macro_apply() {
    assert_eq!(unnamed_term!(1 2), Term::apply(Term::var(1), Term::var(2)));
    assert_eq!(
        unnamed_term!((3 4)),
        Term::apply(Term::var(3), Term::var(4))
    );

    assert_eq!(
        unnamed_term!(0 1 2),
        Term::apply(Term::apply(Term::var(0), Term::var(1)), Term::var(2))
    );

    assert_eq!(
        unnamed_term!(0 1 2 3),
        Term::apply(
            Term::apply(Term::apply(Term::var(0), Term::var(1)), Term::var(2)),
            Term::var(3)
        )
    );

    assert_eq!(
        unnamed_term!((0 1) 2),
        Term::apply(Term::apply(Term::var(0), Term::var(1)), Term::var(2))
    );

    assert_eq!(
        unnamed_term!(0 (1 2)),
        Term::apply(Term::var(0), Term::apply(Term::var(1), Term::var(2)))
    );
}

#[test]
fn macro_lambda() {
    assert_eq!(unnamed_term!(λ 0), Term::lambda(Term::var(0)));
    assert_eq!(unnamed_term!((λ 0)), Term::lambda(Term::var(0)));
    assert_eq!(unnamed_term!(λ 1), Term::lambda(Term::var(1)));

    assert_eq!(
        unnamed_term!(λ λ 1),
        Term::lambda(Term::lambda(Term::var(1)))
    );

    assert_eq!(
        unnamed_term!(λ (λ λ (0))),
        Term::lambda(Term::lambda(Term::lambda(Term::var(0))))
    );
}

#[test]
fn macro_apply_and_lambda() {
    assert_eq!(
        unnamed_term!(λ 0 1 2),
        Term::lambda(Term::apply(
            Term::apply(Term::var(0), Term::var(1)),
            Term::var(2)
        ))
    );

    assert_eq!(
        unnamed_term!((λ 0) 1 2),
        Term::apply(
            Term::apply(Term::lambda(Term::var(0)), Term::var(1)),
            Term::var(2)
        )
    );

    assert_eq!(
        unnamed_term!(λ 0 (λ 0)),
        Term::lambda(Term::apply(Term::var(0), Term::lambda(Term::var(0))))
    );

    assert_eq!(
        unnamed_term!((λ 0) (λ 0)),
        Term::apply(Term::lambda(Term::var(0)), Term::lambda(Term::var(0)))
    );
}
