use std::fmt::{self, Debug, Display};

/// Represents additional options that can be passed to `create_tree` or
/// `create_path`. If options.verbose is set, print out the creation of the file
/// or directory. If options.dry_run is set, print out the creation of the file
/// or directory, but don't actually create it (`options.dry_run` implies
/// verbose)
#[derive(Clone, Copy, Debug)]
pub struct UntreeOptions {
    pub dry_run: bool,
    pub verbose: bool,
}

impl UntreeOptions {
    /// Check if either `self.verbose` or `self.dry_run` is true.
    /// If `dry_run` is true, then `verbose` should be implied as true
    pub fn is_verbose(&self) -> bool {
        self.verbose || self.dry_run
    }
}

/// Enum used to indicate that a path should be created as a file
/// (`PathKind::FilePath`) or a directory (`PathKind::Directory`)
#[derive(Clone, Copy, Debug)]
pub enum PathKind {
    FilePath,
    Directory,
}

impl Display for PathKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for UntreeOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
