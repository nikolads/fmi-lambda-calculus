/// Макрос за създаване на термове литерали.
///
/// Позволява създаване на ламбда термове със синтаксис подобен на
/// математическата нотация. По време на компилация макроса се заемства
/// със съответния Rust код.
///
/// # Пример
///
/// ```
/// use hw::unnamed::Term;
/// use hw::unnamed_term;
///
/// assert_eq!(
///     unnamed_term!(λ 0 1),
///     Term::lambda(Term::apply(Term::var(0), Term::var(1)))
/// );
/// ```
#[macro_export]
macro_rules! unnamed_term {
    // handle '(M)'
    ( ( $($M:tt)+ ) ) => {
        $crate::unnamed_term!($($M)+)
    };

    // handle 'x'
    ($x:tt) => {
        $crate::unnamed::Term::var($x)
    };

    // handle 'λ M'
    (λ $($M:tt)+) => {
        $crate::unnamed::Term::lambda($crate::unnamed_term!($($M)+))
    };

    // handle 'M N ...'
    ( $($tt:tt)+ ) => {
        $crate::_unnamed_parse_apply!({$($tt)+}, [])
    };
}

#[macro_export]
macro_rules! _unnamed_parse_apply {
    // This can happen in a case like 'x λ y'.
    // It could be parsed, but it looks ambigious, so it's better to return an error.
    ( { λ $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        compile_error!("found 'λ' inside list of applications, surround the λ-abstraction with parentheses")
    };

    // '{(M) ...}, [...]' => ' {...}, [{(M)} ...]'
    ( { ( $($M:tt)+ ) $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        $crate::_unnamed_parse_apply!({ $($rest)* }, [{($($M)+)} $({$($tt)*})*])
    };

    // '{x ...}, [...]' => '{...}, [{x} ...]'
    ( { $x:tt $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        $crate::_unnamed_parse_apply!({ $($rest)* }, [{$x} $({$($tt)*})*])
    };

    // handle '{}, [{A1} {A2}]'
    ( {}, [{$($arg1:tt)+} {$($arg2:tt)+}]) => {
        $crate::unnamed::Term::apply(unnamed_term!($($arg2)+), unnamed_term!($($arg1)+))
    };

    // handle '{}, [{A1} ...]'
    ( {}, [{$($arg1:tt)+} $({$($rest:tt)+})*]) => {
        $crate::unnamed::Term::apply($crate::_unnamed_parse_apply!({}, [$({$($rest)+})*]), unnamed_term!($($arg1)+))
    };
}

#[cfg(test)]
mod tests;
