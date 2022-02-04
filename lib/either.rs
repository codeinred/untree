/// Takes a series of expressions returning `Option<T>`, and evaluates each one
/// until finding an expression that returns `Some(item)`
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
