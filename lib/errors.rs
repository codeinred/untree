use quick_error::quick_error;
use std::{io, path::Path, path::PathBuf};

use super::*;

/// Tag type used to indicate that the context for an error was standard input
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadStdinType {
    ReadStdin,
}
pub use ReadStdinType::ReadStdin;

quick_error! {
    /// Error type for untree. Provides additional context to errors, such as
    /// the path type.
    #[derive(Debug)]
    pub enum Error {
        /// Context is missing. An io::Error exists, but the context associated
        /// with the error (where it came from) wasn't known in the current
        /// scope. This context should be provided by a calling function via
        /// error.more_context
        MissingContext(err : io::Error) {
            from()
        }
        /// Error generated while reading from standard input
        OnStdin(err : io::Error) {
            context(_info: ReadStdinType, err: io::Error)
                -> (err)
        }
        /// Error generated while doing some action on the given path
        OnPath(filename: PathBuf, action: PathAction, err: io::Error) {
            context(info: PathContext<'a>, err: io::Error)
                -> (info.0.to_path_buf(), info.1, err)
        }
    }
}

impl MoreContext<ReadStdinType> for Error {
    fn more_context(self, _: ReadStdinType) -> Self {
        use Error::*;
        match self {
            MissingContext(err) => OnStdin(err),
            err => err,
        }
    }
}

impl<'a> MoreContext<(&'a Path, PathAction)> for Error {
    fn more_context(self, (path, action): (&'a Path, PathAction)) -> Self {
        use Error::*;
        match self {
            MissingContext(err) => OnPath(path.to_path_buf(), action, err),
            err => err,
        }
    }
}

/// untree::Result<T> is a std::result::Result<T, untree::Error>. Represents a
/// result type in untree
pub type Result<T> = std::result::Result<T, Error>;
