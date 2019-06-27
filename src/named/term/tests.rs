use super::*;
use crate::{term, unnamed_term};

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
    assert_eq!(Term::from_unnamed(&unnamed_term!(0)), term!(a));
    assert_eq!(Term::from_unnamed(&unnamed_term!(0 1)), term!(a b));
    assert_eq!(Term::from_unnamed(&unnamed_term!(2)), term!(c));

    assert_eq!(Term::from_unnamed(&unnamed_term!(λ 0)), term!(λ x x));
    assert_eq!(Term::from_unnamed(&unnamed_term!(λ 1)), term!(λ x a));

    assert_eq!(
        Term::from_unnamed(&unnamed_term!(λ λ 0)),
        term!(λ x λ y y)
    );
    assert_eq!(
        Term::from_unnamed(&unnamed_term!(λ λ 1)),
        term!(λ x λ y x)
    );
    assert_eq!(
        Term::from_unnamed(&unnamed_term!(λ λ 2)),
        term!(λ x λ y a)
    );

    assert_eq!(
        Term::from_unnamed(&unnamed_term!((λ 0) (λ 0))),
        term!((λ x x) (λ x x))
    );
}

#[test]
fn substitution() {
    assert_eq!(term!(x).substitute("x", &term!(z)), term!(z));
    assert_eq!(term!(x).substitute("y", &term!(z)), term!(x));
    assert_eq!(term!(x y z).substitute("y", &term!(z)), term!(x z z));

    assert_eq!(term!(λ x x).substitute("x", &term!(y)), term!(λ x x));
    assert_eq!(term!(λ x y).substitute("y", &term!(z)), term!(λ x z));
    assert_eq!(term!(λ x y).substitute("y", &term!(x)), term!(λ z x));
    assert_eq!(
        term!(λ x y (λ y x y)).substitute("y", &term!(x)),
        term!(λ z x (λ y z y))
    );

    assert_eq!(
        term!(λ x x y).substitute("y", &term!(x z)),
        term!(λ w w (x z))
    );

    assert_eq!(
        term!(λ x y).substitute("y", &term!(λ x x)),
        term!(λ x (λ x x))
    );
}
