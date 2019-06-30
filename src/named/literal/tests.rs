use crate::named::Term;
use crate::term;

#[test]
fn macro_var() {
    assert_eq!(term!(x), Term::var("x"));
    assert_eq!(term!((y)), Term::var("y"));
    assert_eq!(term!((z)), Term::var("z"));
    assert_eq!(term!(VAR_1), Term::var("VAR_1"));
}

#[test]
fn macro_apply() {
    assert_eq!(term!(a b), Term::apply(Term::var("a"), Term::var("b")));
    assert_eq!(term!((a b)), Term::apply(Term::var("a"), Term::var("b")));

    assert_eq!(
        term!(a b c),
        Term::apply(Term::apply(Term::var("a"), Term::var("b")), Term::var("c"))
    );

    assert_eq!(
        term!(a b c d),
        Term::apply(
            Term::apply(Term::apply(Term::var("a"), Term::var("b")), Term::var("c")),
            Term::var("d")
        )
    );

    assert_eq!(
        term!((a b) c),
        Term::apply(Term::apply(Term::var("a"), Term::var("b")), Term::var("c"))
    );

    assert_eq!(
        term!(a (b c)),
        Term::apply(Term::var("a"), Term::apply(Term::var("b"), Term::var("c")))
    );
}

#[test]
fn macro_lambda() {
    assert_eq!(term!(λ y. y), Term::lambda("y", Term::var("y")));
    assert_eq!(term!((λ y. y)), Term::lambda("y", Term::var("y")));
    assert_eq!(term!(λ VAR_1. x), Term::lambda("VAR_1", Term::var("x")));

    assert_eq!(
        term!(λ a. λ b. a),
        Term::lambda("a", Term::lambda("b", Term::var("a")))
    );

    assert_eq!(
        term!(λ a. (λ b. λ c. (c))),
        Term::lambda("a", Term::lambda("b", Term::lambda("c", Term::var("c"))))
    );
}

#[test]
fn macro_apply_and_lambda() {
    assert_eq!(
        term!(λ x. x b c),
        Term::lambda(
            "x",
            Term::apply(Term::apply(Term::var("x"), Term::var("b")), Term::var("c"))
        )
    );

    assert_eq!(
        term!((λ x. x) b c),
        Term::apply(
            Term::apply(Term::lambda("x", Term::var("x")), Term::var("b")),
            Term::var("c")
        )
    );

    assert_eq!(
        term!(λ x. x (λ y. y)),
        Term::lambda(
            "x",
            Term::apply(Term::var("x"), Term::lambda("y", Term::var("y")))
        )
    );

    assert_eq!(
        term!((λ x. x) (λ y. y)),
        Term::apply(
            Term::lambda("x", Term::var("x")),
            Term::lambda("y", Term::var("y"))
        )
    );
}
