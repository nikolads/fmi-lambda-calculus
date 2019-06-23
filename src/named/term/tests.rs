use super::*;
use crate::named::lambda;
use crate::unnamed::lambda as u_lambda;

#[test]
fn lexographical_names() {
    let gen = LexographicalNames::new(&['a', 'b', 'c']);

    (0..15)
        .map(|i| gen.get(i))
        .zip(&[
            "", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab",
        ])
        .for_each(|(gen, &expected)| assert_eq!(gen, expected));
}

#[test]
fn conversion() {
    assert_eq!(Term::from_unnamed(&u_lambda!(0)), lambda!(a));
    assert_eq!(Term::from_unnamed(&u_lambda!(0 1)), lambda!(a b));
    assert_eq!(Term::from_unnamed(&u_lambda!(2)), lambda!(c));

    assert_eq!(Term::from_unnamed(&u_lambda!(λ 0)), lambda!(λ x x));
    assert_eq!(Term::from_unnamed(&u_lambda!(λ 1)), lambda!(λ x a));

    assert_eq!(
        Term::from_unnamed(&u_lambda!(λ λ 0)),
        lambda!(λ x λ y y)
    );
    assert_eq!(
        Term::from_unnamed(&u_lambda!(λ λ 1)),
        lambda!(λ x λ y x)
    );
    assert_eq!(
        Term::from_unnamed(&u_lambda!(λ λ 2)),
        lambda!(λ x λ y a)
    );

    assert_eq!(
        Term::from_unnamed(&u_lambda!((λ 0) (λ 0))),
        lambda!((λ x x) (λ x x))
    );
}
