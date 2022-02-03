use super::Collapse;
/// Allows context to be added to a value, transforming it into a new type that
/// stores the context.
pub trait AddContext<Context, Source: Collapse<Context>, ResultType> {
    fn add_context(self, context: Source) -> ResultType;
}

macro_rules! add_context {
    ($self:expr, $context:expr) => {
        match $self {
            Ok(value) => Ok(value),
            Err(err) => return Err(err).add_context($context)
        }
    } 
}
pub(crate) use add_context;
