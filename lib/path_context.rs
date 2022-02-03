use super::{Collapse, ContextError, SupplyMissing};
use std::fmt::{self, Debug, Display};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PathContext<PathType> {
    Stdin,
    Missing,
    Read(PathType),
    CreateFile(PathType),
    CreateDirectory(PathType),
}

#[allow(non_upper_case_globals)]
pub const FromStdin: PathContext<PathBuf> = PathContext::Stdin;

impl Display for PathContext<PathBuf> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PathContext::*;
        match self {
            Read(path) | CreateFile(path) | CreateDirectory(path) => {
                let path = path.as_os_str().to_str().unwrap_or("<unprintable>");
                write!(f, "path = '{path}'")
            }
            Stdin => write!(f, "source = standard input"),
            Missing => write!(f, "[unknown]"),
        }
    }
}

impl<T> Collapse<PathContext<PathBuf>> for PathContext<T>
where
    T: Into<PathBuf>,
{
    fn collapse(self) -> PathContext<PathBuf> {
        use PathContext::*;
        match self {
            Read(path) => Read(path.into()),
            CreateFile(path) => CreateFile(path.into()),
            CreateDirectory(path) => CreateDirectory(path.into()),
            Stdin => Stdin,
            Missing => Missing,
        }
    }
}

impl<T> SupplyMissing<PathContext<T>> for PathContext<T> {
    fn supply_missing(self, context: PathContext<T>) -> PathContext<T> {
        match self {
            Self::Missing => context,
            x => x,
        }
    }
}

pub type PathError = ContextError<PathContext<PathBuf>, std::io::Error>;
pub type PathResult<T> = Result<T, PathError>;

impl From<std::io::Error> for PathError {
    fn from(err : std::io::Error) -> PathError {
        PathError {
            context: PathContext::Missing,
            base_error: err
        }
    }
}
