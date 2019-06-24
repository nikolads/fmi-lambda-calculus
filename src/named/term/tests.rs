use super::*;
use crate::named::lambda;
use crate::unnamed::lambda as u_lambda;

#[test]
fn lexicographical_names() {
    let gen = LexicographicalNames::new(&['a', 'b', 'c']);

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

#[test]
fn substitution() {
    assert_eq!(lambda!(x).substitute("x", &lambda!(z)), lambda!(z));
    assert_eq!(lambda!(x).substitute("y", &lambda!(z)), lambda!(x));
    assert_eq!(lambda!(x y z).substitute("y", &lambda!(z)), lambda!(x z z));

    assert_eq!(
        lambda!(λ x x).substitute("x", &lambda!(y)),
        lambda!(λ x x)
    );
    assert_eq!(
        lambda!(λ x y).substitute("y", &lambda!(z)),
        lambda!(λ x z)
    );
    assert_eq!(
        lambda!(λ x y).substitute("y", &lambda!(x)),
        lambda!(λ z x)
    );
    assert_eq!(
        lambda!(λ x y (λ y x y)).substitute("y", &lambda!(x)),
        lambda!(λ z x (λ y z y))
    );

    assert_eq!(
        lambda!(λ x x y).substitute("y", &lambda!(x z)),
        lambda!(λ w w (x z))
    );

    assert_eq!(
        lambda!(λ x y).substitute("y", &lambda!(λ x x)),
        lambda!(λ x (λ x x))
    );
}
