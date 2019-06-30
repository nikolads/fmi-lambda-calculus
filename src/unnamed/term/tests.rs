use super::*;
use crate::{term, unnamed_term};

#[test]
fn conversion() {
    assert_eq!(
        Term::from_named(&term!(x)),
        (unnamed_term!(0), vec![String::from("x")])
    );
    assert_eq!(
        Term::from_named(&term!(x y)),
        (
            unnamed_term!(0 1),
            vec![String::from("x"), String::from("y")]
        )
    );

    assert_eq!(
        Term::from_named(&term!(λ x x)),
        (unnamed_term!(λ 0), vec![])
    );
    assert_eq!(
        Term::from_named(&term!(λ x y)),
        (unnamed_term!(λ 1), vec![String::from("y")])
    );
    assert_eq!(
        Term::from_named(&term!(λ x x y)),
        (unnamed_term!(λ 0 1), vec![String::from("y")])
    );
    assert_eq!(
        Term::from_named(&term!(λ x y x)),
        (unnamed_term!(λ 1 0), vec![String::from("y")])
    );

    assert_eq!(
        Term::from_named(&term!(λ x λ y x)),
        (unnamed_term!(λ λ 1), vec![])
    );
    assert_eq!(
        Term::from_named(&term!(λ x λ y y)),
        (unnamed_term!(λ λ 0), vec![])
    );
    assert_eq!(
        Term::from_named(&term!(λ x λ y z)),
        (unnamed_term!(λ λ 2), vec![String::from("z")])
    );

    assert_eq!(
        Term::from_named(&term!((λ x x) (λ y y))),
        (unnamed_term!((λ 0) (λ 0)), vec![])
    );
    assert_eq!(
        Term::from_named(&term!((λ x x) x)),
        (unnamed_term!((λ 0) 0), vec![String::from("x")])
    );
    assert_eq!(
        Term::from_named(&term!((λ x y) x)),
        (
            unnamed_term!((λ 1) 1),
            vec![String::from("y"), String::from("x")]
        )
    );
    assert_eq!(
        Term::from_named(&term!(x (λ x x))),
        (unnamed_term!(0 (λ 0)), vec![String::from("x")])
    );
    assert_eq!(
        Term::from_named(&term!((λ x λ y x) (λ x x))),
        (unnamed_term!((λ λ 1) (λ 0)), vec![])
    );
}

#[test]
fn substitute_no_match() {
    assert_eq!(
        unnamed_term!(1).substitute(0, &unnamed_term!(2)),
        unnamed_term!(1)
    );

    assert_eq!(
        unnamed_term!(0 2).substitute(1, &unnamed_term!(3)),
        unnamed_term!(0 2)
    );

    assert_eq!(
        unnamed_term!(λ 0).substitute(0, &unnamed_term!(2)),
        unnamed_term!(λ 0)
    );

    assert_eq!(
        unnamed_term!(λ 0 2).substitute(0, &unnamed_term!(2)),
        unnamed_term!(λ 0 2)
    );

    let max = usize::max_value();
    assert_eq!(
        unnamed_term!(λ 0 max).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 max)
    );
}

#[test]
fn substitute_with_var() {
    assert_eq!(
        unnamed_term!(0).substitute(0, &unnamed_term!(2)),
        unnamed_term!(2)
    );

    assert_eq!(
        unnamed_term!(0 1 2).substitute(1, &unnamed_term!(2)),
        unnamed_term!(0 2 2)
    );

    assert_eq!(
        unnamed_term!(λ 1).substitute(0, &unnamed_term!(2)),
        unnamed_term!(λ 3)
    );

    assert_eq!(
        unnamed_term!(λ 0 1 2).substitute(0, &unnamed_term!(2)),
        unnamed_term!(λ 0 3 2)
    );
}

#[test]
fn substitute_with_application() {
    assert_eq!(
        unnamed_term!(0).substitute(0, &unnamed_term!(0 1 2)),
        unnamed_term!(0 1 2)
    );

    assert_eq!(
        unnamed_term!(0 1 2).substitute(1, &unnamed_term!(0 1 2)),
        unnamed_term!(0 (0 1 2) 2)
    );

    assert_eq!(
        unnamed_term!(λ 1).substitute(0, &unnamed_term!(0 1 2)),
        unnamed_term!(λ 1 2 3)
    );

    assert_eq!(
        unnamed_term!(λ 0 1 2).substitute(0, &unnamed_term!(0 1 2)),
        unnamed_term!(λ 0 (1 2 3) 2)
    );
}

#[test]
fn substitute_with_lambda() {
    assert_eq!(
        unnamed_term!(0).substitute(0, &unnamed_term!(λ 0)),
        unnamed_term!(λ 0)
    );

    assert_eq!(
        unnamed_term!(0).substitute(0, &unnamed_term!(λ 0 1)),
        unnamed_term!(λ 0 1)
    );

    assert_eq!(
        unnamed_term!(0 1 2).substitute(1, &unnamed_term!(λ 0 1)),
        unnamed_term!(0 (λ 0 1) 2)
    );

    assert_eq!(
        unnamed_term!(λ 1).substitute(0, &unnamed_term!(λ 0 1)),
        unnamed_term!(λ λ 0 2)
    );

    assert_eq!(
        unnamed_term!(λ 0 1 2).substitute(0, &unnamed_term!(λ 0 1 2)),
        unnamed_term!(λ 0 (λ 0 2 3) 2)
    );
}
