use super::*;
use crate::unnamed::lambda;

#[test]
fn substitution_simple() {
    assert_eq!(substitute(&lambda!(0), 0, &lambda!(2)), lambda!(2));
    assert_eq!(substitute(&lambda!(1), 0, &lambda!(2)), lambda!(1));
    assert_eq!(substitute(&lambda!(0 1 2), 1, &lambda!(2)), lambda!(0 2 2));
}

#[test]
fn substitution_lambda_simple() {
    // (λ 0)[0 -> 1] = (λ 0)
    assert_eq!(substitute(&lambda!(λ 0), 0, &lambda!(1)), lambda!(λ 0));

    // (λ 0 1)[0 -> 1] = (λ 0 2)
    assert_eq!(substitute(&lambda!(λ 0 1), 0, &lambda!(1)), lambda!(λ 0 2));

    // (λ 0 2)[0 -> 1] = (λ 0 2)
    assert_eq!(substitute(&lambda!(λ 0 2), 0, &lambda!(1)), lambda!(λ 0 2));

    // (λ 0 3)[0 -> 1] = (λ 0 3)
    assert_eq!(substitute(&lambda!(λ 0 3), 0, &lambda!(1)), lambda!(λ 0 3));

    // (λ 0 1)[0 -> (1 2)] = (λ 0 (2 3))
    assert_eq!(
        substitute(&lambda!(λ 0 1), 0, &lambda!(1 2)),
        lambda!(λ 0 (2 3))
    );

    // (λ 0 1)[0 -> (λ 1)] = (λ 0 (λ 2))
    assert_eq!(
        substitute(&lambda!(λ 0 1), 0, &lambda!(λ 1)),
        lambda!(λ 0 (λ 2))
    );
}

#[test]
fn substitution_lambda_nested() {
    // (λ 0 (λ 2))[0 -> 1] = (λ 0 (λ 3))
    assert_eq!(
        substitute(&lambda!(λ 0 (λ 2)), 0, &lambda!(1)),
        lambda!(λ 0 (λ 3))
    );
}

#[test]
fn substitution_lambda_overflow() {
    // (λ 0 max)[0 -> 1] = (λ 0 max)
    let max = usize::max_value();
    assert_eq!(substitute(&lambda!(λ 0 max), 0, &lambda!(1)), lambda!(λ 0 max));
}
