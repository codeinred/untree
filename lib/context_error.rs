use std::error::Error;
use std::fmt::{self, Debug, Display};

use super::{AddContext, Collapse, SupplyMissing};

#[derive(Debug)]
pub struct ContextError<T: Debug + Display, E: Error> {
    pub context: T,
    pub base_error: E,
}

impl<T, E> Display for ContextError<T, E>
where
    T: Debug + Display,
    E: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let context = &self.context;
        let base_error = &self.base_error;
        write!(
            f,
            "Error: ContextError {{ context: {context}, base_error: {base_error} }}"
        )
    }
}

impl<T: Debug + Display, E: Error> Error for ContextError<T, E> {}

impl<E, C, T, Source> AddContext<C, Source, Result<T, ContextError<C, E>>> for Result<T, E>
where
    C: Debug + Display,
    E: Error,
    Source: Collapse<C>,
{
    fn add_context(self, context: Source) -> Result<T, ContextError<C, E>> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(ContextError {
                context: context.collapse(),
                base_error: err,
            }),
        }
    }
}

impl<T, E, Info> SupplyMissing<Info> for ContextError<T, E>
where
    E: Error,
    T: Debug + Display + SupplyMissing<Info>,
{
    fn supply_missing(self, info: Info) -> Self {
        Self {
            context: self.context.supply_missing(info),
            base_error: self.base_error,
        }
    }
}
