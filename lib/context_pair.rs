
#[derive(Debug)]
pub struct ContextPair<A: Debug + Display, B: Debug + Display> {
    first: A,
    rest: B,
}

impl<C1, C2> Display for ContextPair<C1, C2>
where
    C1: Debug + Display,
    C2: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let first = &self.first;
        let rest = &self.rest;
        write!(f, "{first}, also: {rest}")
    }
}

impl<E, C1, C2, T, Source>
    AddContext<C2, Source, Result<T, ContextError<ContextPair<C1, C2>, E>>>
    for Result<T, ContextError<C1, E>>
where
    C1: Debug + Display,
    C2: Debug + Display,
    E: Error,
    Source: Into<C2>,
{
    fn add_context(
        self,
        context: Source,
    ) -> Result<T, ContextError<ContextPair<C1, C2>, E>> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(ContextError {
                context: ContextPair {
                    first: err.context,
                    rest: context.into(),
                },
                base_error: err.base_error,
            }),
        }
    }
}
