use std::{fmt, path::Path};

/// Type representing the context for a error. Contains a path, together with a
/// PathAction indicating what was going on at the time of the error
pub type PathContext<'a> = (&'a Path, PathAction);

/// Represents a type of action that can happen on a path. Used to provide
/// additional context to errors, by describing what was happening when the
/// error occured.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PathAction {
    /// Error occured when creating a file for a given path
    CreateFile,
    /// Error occured when creating a directory for a given path
    CreateDirectory,
    /// Error occured while opening a file for reading
    OpenFileForReading,
    /// Error occured while reading file
    ReadFile,
}

pub use PathAction::*;

impl PathAction {
    /// Constructs a PathContext given itself, and the given context
    pub fn on<'a>(self, path: &'a Path) -> PathContext<'a> {
        (path, self)
    }
    /// Describes a path in terms of the action that was occuring on the path
    pub fn describe(self, path: &impl fmt::Display) -> String {
        match self {
            CreateFile => format!("create file '{path}'"),
            CreateDirectory => format!("create directory '{path}'"),
            OpenFileForReading => format!("open '{path}' for reading"),
            ReadFile => format!("read file '{path}'"),
        }
    }
}
