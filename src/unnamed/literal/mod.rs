// Looks like there are problems with macros 2.0 counting items as used
#![allow(unused_imports)]
#![allow(unused_macros)]

use super::Term;

pub macro lambda {
    // handle '(M)'
    ( ( $($M:tt)+ ) ) => {
        lambda!($($M)+)
    },

    // handle 'x'
    ($x:tt) => {
        Term::var($x)
    },

    // handle 'λ M'
    (λ $($M:tt)+) => {
        Term::lambda(lambda!($($M)+))
    },

    // handle 'M N ...'
    ( $($tt:tt)+ ) => {
        parse_apply!({$($tt)+}, [])
    }
}

macro parse_apply {
    // This can happen in a case like 'x λ y'.
    // It could be parsed, but it looks ambigious, so it's better to return an error.
    ( { λ $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        compile_error!("found 'λ' inside list of applications, surround the λ-abstraction with parentheses")
    },

    // '{(M) ...}, [...]' => ' {...}, [{(M)} ...]'
    ( { ( $($M:tt)+ ) $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        parse_apply!({ $($rest)* }, [{($($M)+)} $({$($tt)*})*])
    },

    // '{x ...}, [...]' => '{...}, [{x} ...]'
    ( { $x:tt $($rest:tt)* }, [$({$($tt:tt)*})*]) => {
        parse_apply!({ $($rest)* }, [{$x} $({$($tt)*})*])
    },

    // handle '{}, [{A1} {A2}]'
    ( {}, [{$($arg1:tt)+} {$($arg2:tt)+}]) => {
        Term::apply(lambda!($($arg2)+), lambda!($($arg1)+))
    },

    // handle '{}, [{A1} ...]'
    ( {}, [{$($arg1:tt)+} $({$($rest:tt)+})*]) => {
        Term::apply(parse_apply!({}, [$({$($rest)+})*]), lambda!($($arg1)+))
    }
}

#[cfg(test)]
mod tests;
