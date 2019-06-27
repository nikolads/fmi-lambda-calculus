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
fn substitution_simple() {
    assert_eq!(
        unnamed_term!(0).substitute(0, &unnamed_term!(2)),
        unnamed_term!(2)
    );
    assert_eq!(
        unnamed_term!(1).substitute(0, &unnamed_term!(2)),
        unnamed_term!(1)
    );
    assert_eq!(
        unnamed_term!(0 1 2).substitute(1, &unnamed_term!(2)),
        unnamed_term!(0 2 2)
    );

    // (λ 0)[0 -> 1] = (λ 0)
    assert_eq!(
        unnamed_term!(λ 0).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0)
    );

    // (λ 0 1)[0 -> 1] = (λ 0 2)
    assert_eq!(
        unnamed_term!(λ 0 1).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 2)
    );

    // (λ 0 2)[0 -> 1] = (λ 0 2)
    assert_eq!(
        unnamed_term!(λ 0 2).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 2)
    );

    // (λ 0 3)[0 -> 1] = (λ 0 3)
    assert_eq!(
        unnamed_term!(λ 0 3).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 3)
    );

    // (λ 0 1)[0 -> (1 2)] = (λ 0 (2 3))
    assert_eq!(
        unnamed_term!(λ 0 1).substitute(0, &unnamed_term!(1 2)),
        unnamed_term!(λ 0 (2 3))
    );

    // (λ 0 1)[0 -> (λ 1)] = (λ 0 (λ 2))
    assert_eq!(
        unnamed_term!(λ 0 1).substitute(0, &unnamed_term!(λ 1)),
        unnamed_term!(λ 0 (λ 2))
    );

    // (λ 0 (λ 2))[0 -> 1] = (λ 0 (λ 3))
    assert_eq!(
        unnamed_term!(λ 0 (λ 2)).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 (λ 3))
    );

    // (λ 0 max)[0 -> 1] = (λ 0 max)
    let max = usize::max_value();
    assert_eq!(
        unnamed_term!(λ 0 max).substitute(0, &unnamed_term!(1)),
        unnamed_term!(λ 0 max)
    );
}
