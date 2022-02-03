use super::ContextError;
use std::fmt::{self, Display, Debug};

#[derive(Debug, Clone)]
pub struct PathContext {
    path: std::path::PathBuf,
}

impl Display for PathContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path.as_os_str().to_str().unwrap_or("<unprintable>");
        write!(f, "path = '{path}'")
    }
}

impl<T> From<T> for PathContext
where
    T: Into<std::path::PathBuf>,
{
    fn from(thing: T) -> PathContext {
        PathContext { path: thing.into() }
    }
}

pub type PathError = ContextError<PathContext, std::io::Error>;
pub type PathResult<T> = Result<T, PathError>;
