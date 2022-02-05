use std::fmt::{self, Debug, Display};

/// Represents additional options that can be passed to `create_tree` or
/// `create_path`. If options.verbose is set, print out the creation of the file
/// or directory. If options.dry_run is set, print out the creation of the file
/// or directory, but don't actually create it (`options.dry_run` implies
/// verbose)
#[derive(Clone, Copy, Debug, Default)]
pub struct UntreeOptions {
    pub(crate) dry_run: bool,
    pub(crate) verbose: bool,
}

impl UntreeOptions {
    /// Create a new [`UntreeOptions`] with dry_run and verbose both set to false.
    /// These are the defaults.
    pub fn new() -> Self {
        Default::default()
    }

    /// Return a new [`UntreeOptions`] with dry_run set to the given value
    #[must_use]
    pub fn dry_run(self, dry_run: bool) -> UntreeOptions {
        Self { dry_run, ..self }
    }

    /// Return a new [`UntreeOptions`] with verbose set to the given value
    #[must_use]
    pub fn verbose(self, verbose: bool) -> UntreeOptions {
        Self { verbose, ..self }
    }

    /// Check whether this option specifies that untree should print out what it's doing, without actually making any files or directories
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
    /// Check whether untree should describe what it's doing.
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
