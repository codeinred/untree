use quick_error::{quick_error, ResultExt, Context};
use std::io;
use std::path::{Path, PathBuf};

pub enum Actions {
    ReadStdin, ReadFile, CreateFile, CreateDir, 
}
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
