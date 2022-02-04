/// Sometimes an error can be missing external context. For example, when trying
/// to read a `Lines` struct, the function may not know what the file or stream
/// was that was the source of the `Lines`. In thic case, it'll report a missing
/// context. Invoking `more_context` on the result or the error allows
/// additional context to be filled in by the calling function
pub trait MoreContext<T> {
    /// If a result or error is missing context, provide the missing context
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
