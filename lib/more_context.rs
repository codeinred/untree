/// Trait that allows callers to add missing info if necessary
pub trait MoreContext<T> {
    fn more_context(self, info: T) -> Self;
}

impl<T, E, Info> MoreContext<Info> for Result<T, E>
where
    E: MoreContext<Info>,
{
    fn more_context(self, info: Info) -> Self {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(err.more_context(info)),
        }
    }
}
