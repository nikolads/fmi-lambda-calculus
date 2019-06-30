#[macro_export]
macro_rules! term {
    // handle 'x'
    ($x:ident) => {
        $crate::named::Term::var(stringify!($x))
    };

    // handle 'λ x. M'
    (λ $x:ident . $($M:tt)+) => {
        $crate::named::Term::lambda(stringify!($x), $crate::term!($($M)+))
    };

    // handle '(M)'
    ( ( $($M:tt)+ ) ) => {
        $crate::term!($($M)+)
    };

    // handle 'M N ...'
    ( $($tt:tt)+ ) => {
        $crate::_named_parse_apply!({$($tt)+}, [])
    };
}

#[macro_export]
macro_rules! _named_parse_apply {
    // This can happen in a case like 'x λ y y'.
    // It could be parsed, but it looks ambigious, so it's better to return an error.
    ( { λ $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        compile_error!("found 'λ' inside list of applications, surround the λ-abstraction with parentheses")
    };

    // '{x ...}, [...]' => '{...}, [{x} ...]'
    ( { $x:ident $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        $crate::_named_parse_apply!({ $($rest)* }, [{$x} $({$($tt)*})*])
    };

    // '{(M) ...}, [...]' => ' {...}, [{(M)} ...]'
    ( { ( $($M:tt)+ ) $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        $crate::_named_parse_apply!({ $($rest)* }, [{($($M)+)} $({$($tt)*})*])
    };

    // handle '{}, [{A1} {A2}]'
    ( {}, [{$($arg1:tt)+} {$($arg2:tt)+}]) => {
        $crate::named::Term::apply($crate::term!($($arg2)+), $crate::term!($($arg1)+))
    };

    // handle '{}, [{A1} ...]'
    ( {}, [{$($arg1:tt)+} $({$($rest:tt)+})*]) => {
        $crate::named::Term::apply($crate::_named_parse_apply!({}, [$({$($rest)+})*]), $crate::term!($($arg1)+))
    };
}

#[cfg(test)]
mod tests;
