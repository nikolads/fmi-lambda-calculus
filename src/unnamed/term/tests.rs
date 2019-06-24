use super::*;
use crate::unnamed::lambda;
use crate::named::lambda as named_lambda;

#[test]
fn conversion() {
    assert_eq!(Term::from_named(&named_lambda!(x)), (lambda!(0), vec![String::from("x")]));
    assert_eq!(Term::from_named(&named_lambda!(x y)), (lambda!(0 1), vec![String::from("x"), String::from("y")]));

    assert_eq!(Term::from_named(&named_lambda!(λ x x)), (lambda!(λ 0), vec![]));
    assert_eq!(Term::from_named(&named_lambda!(λ x y)), (lambda!(λ 1), vec![String::from("y")]));
    assert_eq!(Term::from_named(&named_lambda!(λ x x y)), (lambda!(λ 0 1), vec![String::from("y")]));
    assert_eq!(Term::from_named(&named_lambda!(λ x y x)), (lambda!(λ 1 0), vec![String::from("y")]));

    assert_eq!(Term::from_named(&named_lambda!(λ x λ y x)), (lambda!(λ λ 1), vec![]));
    assert_eq!(Term::from_named(&named_lambda!(λ x λ y y)), (lambda!(λ λ 0), vec![]));
    assert_eq!(Term::from_named(&named_lambda!(λ x λ y z)), (lambda!(λ λ 2), vec![String::from("z")]));

    assert_eq!(Term::from_named(&named_lambda!((λ x x) (λ y y))), (lambda!((λ 0) (λ 0)), vec![]));
    assert_eq!(Term::from_named(&named_lambda!((λ x x) x)), (lambda!((λ 0) 0), vec![String::from("x")]));
    assert_eq!(Term::from_named(&named_lambda!((λ x y) x)), (lambda!((λ 1) 1), vec![String::from("y"), String::from("x")]));
    assert_eq!(Term::from_named(&named_lambda!(x (λ x x))), (lambda!(0 (λ 0)), vec![String::from("x")]));
    assert_eq!(Term::from_named(&named_lambda!((λ x λ y x) (λ x x))), (lambda!((λ λ 1) (λ 0)), vec![]));
}

#[test]
fn substitution_simple() {
    assert_eq!(lambda!(0).substitute(0, &lambda!(2)), lambda!(2));
    assert_eq!(lambda!(1).substitute(0, &lambda!(2)), lambda!(1));
    assert_eq!(lambda!(0 1 2).substitute(1, &lambda!(2)), lambda!(0 2 2));

    // (λ 0)[0 -> 1] = (λ 0)
    assert_eq!(lambda!(λ 0).substitute(0, &lambda!(1)), lambda!(λ 0));

    // (λ 0 1)[0 -> 1] = (λ 0 2)
    assert_eq!(lambda!(λ 0 1).substitute(0, &lambda!(1)), lambda!(λ 0 2));

    // (λ 0 2)[0 -> 1] = (λ 0 2)
    assert_eq!(lambda!(λ 0 2).substitute(0, &lambda!(1)), lambda!(λ 0 2));

    // (λ 0 3)[0 -> 1] = (λ 0 3)
    assert_eq!(lambda!(λ 0 3).substitute(0, &lambda!(1)), lambda!(λ 0 3));

    // (λ 0 1)[0 -> (1 2)] = (λ 0 (2 3))
    assert_eq!(
        lambda!(λ 0 1).substitute(0, &lambda!(1 2)),
        lambda!(λ 0 (2 3))
    );

    // (λ 0 1)[0 -> (λ 1)] = (λ 0 (λ 2))
    assert_eq!(
        lambda!(λ 0 1).substitute(0, &lambda!(λ 1)),
        lambda!(λ 0 (λ 2))
    );

    // (λ 0 (λ 2))[0 -> 1] = (λ 0 (λ 3))
    assert_eq!(
        lambda!(λ 0 (λ 2)).substitute(0, &lambda!(1)),
        lambda!(λ 0 (λ 3))
    );

    // (λ 0 max)[0 -> 1] = (λ 0 max)
    let max = usize::max_value();
    assert_eq!(lambda!(λ 0 max).substitute(0, &lambda!(1)), lambda!(λ 0 max));
}
