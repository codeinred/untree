#[allow(unused_macros)]

/**
 * Takes an Option<Result<T, Err>> and produces a Option<T>, with the error
 * being propagated by the ? operator
 */
macro_rules! unwrap {
    ($expression:expr) => {
        match ($expression) {
            Some(result) => Some(result?),
            None => None,
        }
    };
}

// I like this macro and I wanna keep it
#[allow(unused_imports)]
pub(crate) use unwrap;

/**
 * Takes a series of expressions returning Option<T>, and evaluates each one 
 * until finding an expression that succeeds
 */
macro_rules! either {
    ($expression:expr) => { $expression };
    ($first:expr, $($second:expr),+) => {
        match $first {
            Some(item) => Some(item),
            None => either!($($second),+)
        }
    }
}
pub(crate) use either;
