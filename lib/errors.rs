use quick_error::quick_error;
use std::io;
use std::path::{Path, PathBuf};

use super::MoreContext;

pub enum ReadStdinType {
    ReadStdin,
}
pub use ReadStdinType::ReadStdin;

type PathContext<'a> = (&'a Path, PathAction);

#[derive(Debug, Copy, Clone)]
pub enum PathAction {
    CreateFile,
    CreateDirectory,
    OpenFileForReading,
    ReadFile,
}

pub use PathAction::*;

impl PathAction {
    pub fn on<'a>(self, path: &'a Path) -> PathContext<'a> {
        (path, self)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MissingContext(err : io::Error) {
            from()
        }
        OnStdin(err : io::Error) {

        }
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

pub type Result<T> = std::result::Result<T, Error>;
