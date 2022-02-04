use std::{fmt, path::Path};

pub(crate) type PathContext<'a> = (&'a Path, PathAction);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    pub fn describe(self, path : &impl fmt::Display) -> String {
        match self {
            CreateFile => format!("create file '{path}'"),
            CreateDirectory => format!("create directory '{path}'"),
            OpenFileForReading => format!("open '{path}' for reading"),
            ReadFile => format!("read file '{path}'"),
        }
    }
}
