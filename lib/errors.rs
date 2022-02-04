use quick_error::quick_error;
use std::io;
use std::path::{Path, PathBuf};

use super::MoreContext;

pub enum ReadStdinType {
    ReadStdin,
}
pub use ReadStdinType::ReadStdin;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MissingContext(err : io::Error) {
            from()
        }
        OnStdin(err : io::Error) {

        }
        OnPath(filename: PathBuf, err: io::Error) {
            context(path: &'a Path, err: io::Error)
                -> (path.to_path_buf(), err)
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

impl<'a> MoreContext<&'a Path> for Error {
    fn more_context(self, path: &'a Path) -> Self {
        use Error::*;
        match self {
            MissingContext(err) => OnPath(path.to_path_buf(), err),
            err => err,
        }
    }
}
